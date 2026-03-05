# MiyuWeb â€” Unit Tests Contract

## 1. Contexte

Ce document dÃ©finit le contrat pour les **tests unitaires** des Tools du kit MiyuWeb. Les tests unitaires vÃ©rifient le comportement de chaque Tool (rendu HTML, layout, thÃ¨me, script, asset, formulaire, Ã©vÃ©nements) sans modifier les donnÃ©es mÃ©tier et sans dÃ©cision de contenu ni accÃ¨s direct Ã  la base.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les types de tests unitaires sur les Tools MiyuWeb (html.render, layout.render, theme.resolve, script.execute, script.compile, asset.serve, form.validate, event.dispatch, input.capture)
- Les critÃ¨res de succÃ¨s et d'Ã©chec
- La non-destructivitÃ© et l'absence de modification de donnÃ©es mÃ©tier
- Les protocoles de vÃ©rification (sandbox / mocks pour donnÃ©es et assets)

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuWeb - Cycle Tests Contract)
- L'implÃ©mentation technique des tests
- Les tests de cohÃ©rence applicative (voir MiyukiniAdmin - Unit Tests Contract si pertinent)

---

## 3. Principe fondamental

### 3.1 Non-destructivitÃ©

> **Les tests unitaires MiyuWeb vÃ©rifient le comportement des Tools sans modifier les donnÃ©es mÃ©tier. Les tests utilisant des templates ou assets utilisent une sandbox ou des mocks ; aucune donnÃ©e mÃ©tier n'est modifiÃ©e. MiyuWeb ne lit jamais la base directement â€” les donnÃ©es sont fournies dans le flux.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MWEB-1** | Aucune modification des donnÃ©es mÃ©tier (templates ou assets de production) |
| **INV-UT-MWEB-2** | Les tests utilisant templates/assets utilisent une sandbox ou des mocks avec nettoyage obligatoire |
| **INV-UT-MWEB-3** | Aucun accÃ¨s direct Ã  la base par MiyuWeb ; toutes les entrÃ©es sont fournies dans le flux (mock ou sandbox) |
| **INV-UT-MWEB-4** | TraÃ§abilitÃ© complÃ¨te de chaque test (contexte, verdict, durÃ©e) |
| **INV-UT-MWEB-5** | Rapports conservÃ©s pour audit |

---

## 4. CatÃ©gories de tests par Tool

### 4.1 Tests Rendu HTML (tool.web.html.render)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-HR-001** | Rendu template valide | VÃ©rifie qu'un template et des donnÃ©es fournis en flux produisent du HTML conforme | Template et donnÃ©es de test (mock) ; pas de donnÃ©es mÃ©tier |
| **MWEB-HR-002** | Rendu template vide | VÃ©rifie le comportement pour un template vide ou des donnÃ©es vides | DonnÃ©es de test |
| **MWEB-HR-003** | Rendu sans dÃ©cision contenu | VÃ©rifie que le Tool ne choisit pas le contenu ; il rend uniquement ce qui est fourni | DonnÃ©es de test |
| **MWEB-HR-004** | Sanitization sortie | VÃ©rifie que la sortie HTML est conforme aux attentes de sÃ©curitÃ© (pas d'injection brute non gouvernÃ©e) | DonnÃ©es de test contrÃ´lÃ©es |

### 4.2 Tests Rendu Layout (tool.web.layout.render)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-LR-001** | Rendu layout valide | VÃ©rifie qu'un layout (zones, placeholders) et des donnÃ©es fournis produisent une structure conforme | Layout et donnÃ©es de test (mock) |
| **MWEB-LR-002** | Rendu layout partiel | VÃ©rifie le comportement pour des zones manquantes ou partielles | DonnÃ©es de test |
| **MWEB-LR-003** | Pas de dÃ©cision contenu zones | VÃ©rifie que le Tool ne dÃ©cide pas du contenu des zones ; exÃ©cute uniquement le rendu de structure | DonnÃ©es de test |

### 4.3 Tests RÃ©solution thÃ¨me (tool.web.theme.resolve)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-TR-001** | RÃ©solution thÃ¨me valide | VÃ©rifie qu'un contexte (ex. mode clair/sombre, identifiant thÃ¨me) produit des donnÃ©es de thÃ¨me conformes | Contexte de test |
| **MWEB-TR-002** | RÃ©solution thÃ¨me inconnu | VÃ©rifie le comportement pour un identifiant de thÃ¨me inconnu ou absent | DonnÃ©es de test |
| **MWEB-TR-003** | Pas de politique thÃ¨me | VÃ©rifie que le Tool ne dÃ©cide pas de la politique de thÃ¨me ; il rÃ©sout uniquement pour le contexte fourni | DonnÃ©es de test |

### 4.4 Tests ExÃ©cution script (tool.web.script.execute)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-SE-001** | ExÃ©cution script valide | VÃ©rifie qu'un script fourni dans le flux s'exÃ©cute dans un contexte sandboxÃ© et retourne un rÃ©sultat attendu | Script et donnÃ©es de test (mock) |
| **MWEB-SE-002** | Sandbox isolation | VÃ©rifie qu'aucun accÃ¨s direct Ã  la base ni dÃ©cision mÃ©tier n'est possible depuis le script exÃ©cutÃ© | Script de test contrÃ´lÃ© |
| **MWEB-SE-003** | Script invalide | VÃ©rifie le comportement pour un script mal formÃ© ou interdit | DonnÃ©es de test |
| **MWEB-SE-004** | EntrÃ©es fournies dans le flux | VÃ©rifie que les entrÃ©es (donnÃ©es, contexte) sont bien celles fournies dans le flux | DonnÃ©es de test |

### 4.5 Tests Compilation script (tool.web.script.compile)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-SC-001** | Compilation script valide | VÃ©rifie qu'un script fourni est compilÃ© ou validÃ© (syntaxe, types) sans exÃ©cution | Script de test |
| **MWEB-SC-002** | Refus script invalide | VÃ©rifie que un script invalide est rejetÃ© ou signale des erreurs conformes | Script de test |
| **MWEB-SC-003** | Pas d'exÃ©cution | VÃ©rifie que le Tool ne fait que compiler/valider, jamais exÃ©cuter | Script de test |

### 4.6 Tests Service asset (tool.web.asset.serve)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-AS-001** | Service asset valide | VÃ©rifie qu'un asset (contenu ou mÃ©tadonnÃ©es fournis dans le flux) est servi correctement | Asset de test (mock) ; pas de lecture base |
| **MWEB-AS-002** | Asset absent ou vide | VÃ©rifie le comportement pour un asset absent ou vide | DonnÃ©es de test |
| **MWEB-AS-003** | Pas de lecture base | VÃ©rifie que le Tool ne lit pas la base ; l'asset est uniquement celui fourni dans le flux | DonnÃ©es de test |

### 4.7 Tests Validation formulaire (tool.web.form.validate)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-FV-001** | Validation formulaire valide | VÃ©rifie qu'un formulaire (structure, champs) et des rÃ¨gles fournies sont validÃ©s correctement | Formulaire et rÃ¨gles de test |
| **MWEB-FV-002** | Validation formulaire invalide | VÃ©rifie le rejet ou les erreurs pour une structure/champs invalides | DonnÃ©es de test |
| **MWEB-FV-003** | Pas de rÃ¨gles mÃ©tier | VÃ©rifie que le Tool n'introduit pas de rÃ¨gles mÃ©tier ; exÃ©cute une validation gouvernÃ©e sur des rÃ¨gles fournies | DonnÃ©es de test |

### 4.8 Tests Dispatch Ã©vÃ©nement (tool.web.event.dispatch)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-ED-001** | Dispatch Ã©vÃ©nement valide | VÃ©rifie qu'un Ã©vÃ©nement est propagÃ© dans le flux gouvernÃ© conformÃ©ment au contrat | Ã‰vÃ©nement de test |
| **MWEB-ED-002** | Pas de dÃ©cision traitement | VÃ©rifie que le Tool ne dÃ©cide pas du traitement ; il dispatche uniquement | DonnÃ©es de test |

### 4.9 Tests Capture entrÃ©e (tool.web.input.capture)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MWEB-IC-001** | Capture entrÃ©e valide | VÃ©rifie qu'une entrÃ©e utilisateur (clic, saisie, touche) est capturÃ©e et transmise au flux gouvernÃ© | EntrÃ©e de test |
| **MWEB-IC-002** | Pas de dÃ©cision usage | VÃ©rifie que le Tool ne dÃ©cide pas de l'usage ; il capture uniquement | DonnÃ©es de test |

---

## 5. CritÃ¨res de succÃ¨s et d'Ã©chec

### 5.1 CritÃ¨res de succÃ¨s

| CritÃ¨res | Description |
|----------|-------------|
| **ExÃ©cution conforme** | Le Tool s'exÃ©cute comme spÃ©cifiÃ© (pas d'exception non contractuelle) |
| **RÃ©sultat attendu** | Sortie (HTML, layout, thÃ¨me, rÃ©sultat script, asset servi, verdict validation, Ã©vÃ©nement, entrÃ©e) conforme au contrat |
| **Pas de fuite** | Aucune donnÃ©e mÃ©tier exposÃ©e ; sandbox nettoyÃ©e aprÃ¨s test si applicable |
| **TraÃ§abilitÃ©** | Contexte, verdict, durÃ©e enregistrÃ©s |

### 5.2 CritÃ¨res d'Ã©chec

| CritÃ¨res | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool lÃ¨ve une exception non prÃ©vue par le contrat |
| **Modification hors sandbox** | Une donnÃ©e mÃ©tier (template ou asset de production) est modifiÃ©e |
| **AccÃ¨s direct base** | MiyuWeb tente une lecture ou Ã©criture directe en base pendant le test |
| **Nettoyage non effectuÃ©** | La sandbox ou les mocks ne sont pas nettoyÃ©s aprÃ¨s un test |
| **Timeout dÃ©passÃ©** | Le test dÃ©passe le timeout configurÃ© sans rÃ©sultat |

### 5.3 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Tous les critÃ¨res de succÃ¨s sont remplis |
| **WARN** | Comportement conforme avec alertes mineures (ex. performance) |
| **FAIL** | Un ou plusieurs critÃ¨res d'Ã©chec sont remplis |
| **SKIP** | PrÃ©-condition non remplie (ex. sandbox indisponible) |
| **ERROR** | Erreur technique pendant le test (configuration, environnement) |

---

## 6. Protocole de test

### 6.1 ExÃ©cution d'un test unitaire MiyuWeb

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. Chargement dÃ©finition du test                              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - ID du test (MWEB-*)                                         â”‚
â”‚ - ToolId concernÃ©                                             â”‚
â”‚ - ParamÃ¨tres (sandbox, mocks, timeout, etc.)                   â”‚
â”‚ - CritÃ¨res de succÃ¨s                                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. PrÃ©paration (si sandbox / mocks)                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - CrÃ©ation sandbox / mocks (templates, assets, donnÃ©es)       â”‚
â”‚ - Via gouvernance ; aucune lecture directe en base             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. ExÃ©cution du Tool (via gouvernance)                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - BondingBrother â†’ Master Butler â†’ WorrySentinel â†’            â”‚
â”‚   Caring Nanny â†’ StrongFather â†’ MiyuWeb Tool                 â”‚
â”‚ - DonnÃ©es fournies dans le flux ; collecte rÃ©sultat ou exception â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Nettoyage (si sandbox)                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Suppression donnÃ©es test / tear-down sandbox                â”‚
â”‚ - Via gouvernance                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 5. Ã‰valuation et rapport                                      â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Verdict (PASS/WARN/FAIL/SKIP/ERROR)                        â”‚
â”‚ - DÃ©tails, durÃ©e, traÃ§abilitÃ©                                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 7. Suites de tests

### 7.1 Suites prÃ©dÃ©finies

| Suite | Tests inclus | DurÃ©e estimÃ©e | Usage |
|-------|--------------|---------------|-------|
| **Quick** | MWEB-HR-001, MWEB-TR-001, MWEB-ED-001, MWEB-IC-001 | < 1 min | VÃ©rification rapide |
| **Standard** | Tous MWEB-HR, MWEB-LR, MWEB-TR, MWEB-AS, MWEB-FV, MWEB-ED, MWEB-IC (mocks) | 2â€“5 min | VÃ©rification quotidienne |
| **Full** | Tous MWEB-* (avec sandbox pour script.execute / script.compile si applicable) | 5â€“10 min | VÃ©rification complÃ¨te |

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Reference Outils | [MiyuWeb - Reference Outils](../../MiyuWeb%20-%20Reference%20Outils.md) |
| MiyuWeb - Cycle Tests Contract | [MiyuWeb - Cycle Tests Contract](./MiyuWeb%20-%20Cycle%20Tests%20Contract.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

