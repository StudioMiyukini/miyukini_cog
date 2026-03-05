# MiyuClock â€” Unit Tests Contract

## 1. Contexte

Ce document dÃ©finit le contrat pour les **tests unitaires** des Tools du kit MiyuClock. Les tests unitaires vÃ©rifient le comportement de chaque Tool (now, delta) sans persistance, sans dÃ©cision mÃ©tier et sans dÃ©pendance Ã  un temps global (conformitÃ© **LOI-4**).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les types de tests unitaires sur les Tools MiyuClock (tool.time.now, tool.time.delta)
- Les critÃ¨res de succÃ¨s et d'Ã©chec
- La non-destructivitÃ© et l'absence de persistance
- Les protocoles de vÃ©rification (comportement de now : type de sortie, local ; delta : t_prev, t_now â†’ durÃ©e ; pas de dÃ©pendance temps global)

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuClock - Cycle Tests Contract)
- L'implÃ©mentation technique des tests
- La persistance des timestamps (OpÃ©rateur + KindMother/MiyuSQL)

---

## 3. Principe fondamental

### 3.1 Non-destructivitÃ© et absence de persistance

> **Les tests unitaires MiyuClock vÃ©rifient le comportement des Tools sans persister de donnÃ©es. Les valeurs de temps sont produites et consommÃ©es dans le flux de test ; aucune Ã©criture en base. Aucune dÃ©pendance Ã  un temps global (LOI-4).**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MCLOCK-1** | Aucune persistance ; les tests n'Ã©crivent pas de timestamps en base |
| **INV-UT-MCLOCK-2** | Aucune dÃ©pendance Ã  un temps global (NTP, serveur externe) ; horloge locale uniquement |
| **INV-UT-MCLOCK-3** | TraÃ§abilitÃ© complÃ¨te de chaque test (contexte, verdict, durÃ©e) |
| **INV-UT-MCLOCK-4** | Rapports conservÃ©s pour audit |

---

## 4. CatÃ©gories de tests par Tool

### 4.1 Tests Instant prÃ©sent (tool.time.now)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MCLOCK-N-001** | Type de sortie | VÃ©rifie que la sortie de `tool.time.now` est du type attendu (instant / horodatage local) | Aucune ; appel gouvernÃ© |
| **MCLOCK-N-002** | RÃ©fÃ©rence locale | VÃ©rifie que l'instant retournÃ© provient de l'horloge locale (Kernel Clock), pas d'une source externe | Aucune |
| **MCLOCK-N-003** | Pas de timezone imposÃ©e | VÃ©rifie que MiyuClock n'impose pas de timezone ; la valeur est une rÃ©fÃ©rence locale | Aucune |
| **MCLOCK-N-004** | Pas de temps global | VÃ©rifie l'absence de dÃ©pendance Ã  un temps global (LOI-4) ; le test ne requiert pas NTP ni serveur de temps | Aucune |

### 4.2 Tests Delta entre instants (tool.time.delta)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MCLOCK-D-001** | Delta positif | VÃ©rifie que `tool.time.delta(t_prev, t_now)` retourne une durÃ©e positive lorsque t_now > t_prev | t_prev, t_now fournis dans le flux (types compatibles avec sortie de now) |
| **MCLOCK-D-002** | Delta nul | VÃ©rifie que `tool.time.delta(t, t)` retourne une durÃ©e nulle (ou Ã©quivalente) | t fourni dans le flux |
| **MCLOCK-D-003** | Ordre des arguments | VÃ©rifie le comportement documentÃ© lorsque t_prev > t_now (durÃ©e nÃ©gative ou valeur absolue selon contrat d'implÃ©mentation) | t_prev, t_now |
| **MCLOCK-D-004** | Types compatibles | VÃ©rifie que les entrÃ©es (rÃ©fÃ©rences ou sorties de `tool.time.now`) sont acceptÃ©es et que la sortie est une durÃ©e | Instants fournis dans le flux |
| **MCLOCK-D-005** | Pas de temps global | VÃ©rifie que le calcul de delta ne dÃ©pend pas d'un temps global ; uniquement les instants fournis dans le flux | t_prev, t_now |

---

## 5. CritÃ¨res de succÃ¨s et d'Ã©chec

### 5.1 CritÃ¨res de succÃ¨s

| CritÃ¨res | Description |
|----------|-------------|
| **ExÃ©cution conforme** | Le Tool s'exÃ©cute comme spÃ©cifiÃ© (pas d'exception non contractuelle) |
| **RÃ©sultat attendu** | Pour now : type d'instant local conforme ; pour delta : durÃ©e conforme aux entrÃ©es |
| **Pas de persistance** | Aucune Ã©criture en base ; valeurs dans le flux uniquement |
| **Pas de temps global** | Aucune dÃ©pendance Ã  NTP ou serveur de temps externe |
| **TraÃ§abilitÃ©** | Contexte, verdict, durÃ©e enregistrÃ©s |

### 5.2 CritÃ¨res d'Ã©chec

| CritÃ¨res | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool lÃ¨ve une exception non prÃ©vue par le contrat |
| **Persistance effectuÃ©e** | Une Ã©criture en base est effectuÃ©e par MiyuClock pendant le test |
| **DÃ©pendance temps global** | Le test ou le Tool requiert un temps global (NTP, etc.) |
| **Type de sortie incorrect** | now ne retourne pas un type d'instant attendu ; delta ne retourne pas une durÃ©e |
| **Timeout dÃ©passÃ©** | Le test dÃ©passe le timeout configurÃ© sans rÃ©sultat |

### 5.3 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Tous les critÃ¨res de succÃ¨s sont remplis |
| **WARN** | Comportement conforme avec alertes mineures (ex. performance) |
| **FAIL** | Un ou plusieurs critÃ¨res d'Ã©chec sont remplis |
| **SKIP** | PrÃ©-condition non remplie (ex. Kernel Clock indisponible) |
| **ERROR** | Erreur technique pendant le test (configuration, environnement) |

---

## 6. Protocole de test

### 6.1 ExÃ©cution d'un test unitaire MiyuClock

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. Chargement dÃ©finition du test                              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - ID du test (MCLOCK-*)                                       â”‚
â”‚ - ToolId concernÃ© (tool.time.now ou tool.time.delta)          â”‚
â”‚ - ParamÃ¨tres (instants pour delta, timeout, etc.)             â”‚
â”‚ - CritÃ¨res de succÃ¨s                                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. PrÃ©paration (si nÃ©cessaire pour delta)                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Obtenir t_prev, t_now via tool.time.now (gouvernÃ©) ou       â”‚
â”‚   fournir des instants de test compatibles                    â”‚
â”‚ - Aucune persistance                                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. ExÃ©cution du Tool (via gouvernance)                         â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - BondingBrother â†’ Master Butler â†’ WorrySentinel â†’            â”‚
â”‚   Caring Nanny â†’ StrongFather â†’ MiyuClock Tool                â”‚
â”‚ - Collecte rÃ©sultat ou exception                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Ã‰valuation et rapport                                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Verdict (PASS/WARN/FAIL/SKIP/ERROR)                         â”‚
â”‚ - DÃ©tails, durÃ©e, traÃ§abilitÃ©                                â”‚
â”‚ - Aucun nettoyage persistance (MiyuClock ne persiste pas)      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 7. Suites de tests

### 7.1 Suites prÃ©dÃ©finies

| Suite | Tests inclus | DurÃ©e estimÃ©e | Usage |
|-------|--------------|---------------|-------|
| **Quick** | MCLOCK-N-001, MCLOCK-N-002, MCLOCK-D-001, MCLOCK-D-002 | < 1 min | VÃ©rification rapide |
| **Standard** | Tous MCLOCK-N-*, MCLOCK-D-* | 1â€“3 min | VÃ©rification quotidienne |
| **Full** | Tous MCLOCK-* (now + delta, tous cas) | 2â€“5 min | VÃ©rification complÃ¨te |

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Reference Outils | [MiyuClock - Reference Outils](../../MiyuClock%20-%20Reference%20Outils.md) |
| MiyuClock - Cycle Tests Contract | [MiyuClock - Cycle Tests Contract](./MiyuClock%20-%20Cycle%20Tests%20Contract.md) |
| MiyuClock - KindMother Integration Contract | [MiyuClock - KindMother Integration Contract](../integration/MiyuClock%20-%20KindMother%20Integration%20Contract.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

