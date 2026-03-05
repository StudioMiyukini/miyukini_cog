# MiyuAuth â€” Unit Tests Contract

## 1. Contexte

Ce document dÃ©finit le contrat pour les **tests unitaires** des Tools du kit MiyuAuth. Les tests unitaires vÃ©rifient le comportement de chaque Tool (resolve, attest, verify, role) sans modifier les donnÃ©es rÃ©elles et sans dÃ©cision de confiance ni d'autorisation.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les types de tests unitaires sur les Tools MiyuAuth (resolve, attest, verify, role)
- Les critÃ¨res de succÃ¨s et d'Ã©chec
- La non-destructivitÃ© et l'absence de modification de donnÃ©es mÃ©tier
- Les protocoles de vÃ©rification (sandbox / mocks pour attest/verify si nÃ©cessaire)

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuAuth - Cycle Tests Contract)
- L'implÃ©mentation technique des tests
- Les tests de cohÃ©rence applicative (voir MiyukiniAdmin - Unit Tests Contract si pertinent)

---

## 3. Principe fondamental

### 3.1 Non-destructivitÃ©

> **Les tests unitaires MiyuAuth vÃ©rifient le comportement des Tools sans modifier les donnÃ©es mÃ©tier. Les tests d'attestation et de vÃ©rification utilisent une sandbox ou des mocks si nÃ©cessaire ; aucune donnÃ©e mÃ©tier n'est modifiÃ©e.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MAUTH-1** | Aucune modification des donnÃ©es mÃ©tier (identitÃ©s de production, Passeports/Visas rÃ©els) |
| **INV-UT-MAUTH-2** | Les tests d'attestation et de vÃ©rification utilisent une sandbox ou des mocks (Passeport/Visa de test) avec nettoyage obligatoire |
| **INV-UT-MAUTH-3** | TraÃ§abilitÃ© complÃ¨te de chaque test (contexte, verdict, durÃ©e) |
| **INV-UT-MAUTH-4** | Rapports conservÃ©s pour audit |

---

## 4. CatÃ©gories de tests par Tool

### 4.1 Tests RÃ©solution (tool.identity.resolve)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MAUTH-R-001** | RÃ©solution citoyen | VÃ©rifie qu'un contexte citoyen (donnÃ©es fournies) est rÃ©solu correctement | Contexte de test ; pas de donnÃ©es mÃ©tier |
| **MAUTH-R-002** | RÃ©solution visiteur | VÃ©rifie qu'un contexte visiteur (Passeport/Visa de test) est rÃ©solu correctement | Sandbox / mocks ; pas de donnÃ©es mÃ©tier |
| **MAUTH-R-003** | RÃ©solution externe | VÃ©rifie qu'un contexte externe (absence de certificat) est rÃ©solu correctement | Aucune |
| **MAUTH-R-004** | Contexte invalide | VÃ©rifie le comportement pour un contexte invalide ou incomplet | Aucune |

### 4.2 Tests Attestation (tool.identity.attest)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MAUTH-A-001** | Attestation contexte validÃ© | VÃ©rifie qu'une attestation est produite pour un contexte validÃ© par KindMother (mock) | Sandbox / mocks ; pas de donnÃ©es mÃ©tier |
| **MAUTH-A-002** | Attestation contexte non validÃ© | VÃ©rifie le refus d'attestation sans validation KindMother | Aucune |
| **MAUTH-A-003** | Structure attestation | VÃ©rifie que la structure de l'attestation est conforme au contrat | Sandbox / mocks |

### 4.3 Tests VÃ©rification (tool.identity.verify)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MAUTH-V-001** | VÃ©rification Passeport valide | VÃ©rifie qu'un Passeport Utilisateur de test (mock) est reconnu comme valide (structure, signature) | Sandbox / mocks ; pas de donnÃ©es mÃ©tier |
| **MAUTH-V-002** | VÃ©rification Visa valide | VÃ©rifie qu'un Visa de Connexion de test (mock) est reconnu comme valide | Sandbox / mocks |
| **MAUTH-V-003** | VÃ©rification invalide | VÃ©rifie le rejet pour un Passeport/Visa invalide ou mal formÃ© | Aucune |
| **MAUTH-V-004** | VÃ©rification sans confiance | VÃ©rifie que la vÃ©rification technique ne valide pas la confiance (KindMother reste validateur) | Sandbox / mocks |

### 4.4 Tests RÃ´le (tool.identity.role)

| Code | Test | Description | DonnÃ©es |
|------|------|-------------|---------|
| **MAUTH-RO-001** | RÃ´le citoyen | VÃ©rifie que le rÃ´le Â« citoyen Â» est retournÃ© pour un contexte citoyen gouvernÃ© | Contexte de test |
| **MAUTH-RO-002** | RÃ´le visiteur | VÃ©rifie que le rÃ´le Â« visiteur Â» est retournÃ© pour un contexte visiteur gouvernÃ© | Sandbox / mocks |
| **MAUTH-RO-003** | RÃ´le externe | VÃ©rifie que le rÃ´le Â« externe Â» est retournÃ© pour un contexte externe gouvernÃ© | Aucune |
| **MAUTH-RO-004** | Contexte absent | VÃ©rifie le comportement pour un contexte absent ou invalide | Aucune |

---

## 5. CritÃ¨res de succÃ¨s et d'Ã©chec

### 5.1 CritÃ¨res de succÃ¨s

| CritÃ¨res | Description |
|----------|-------------|
| **ExÃ©cution conforme** | Le Tool s'exÃ©cute comme spÃ©cifiÃ© (pas d'exception non contractuelle) |
| **RÃ©sultat attendu** | Pour resolve/role : contexte et rÃ´le conformes ; pour attest/verify : structure et verdict conformes |
| **Pas de fuite** | Aucune donnÃ©e mÃ©tier exposÃ©e ; sandbox nettoyÃ©e aprÃ¨s test si applicable |
| **TraÃ§abilitÃ©** | Contexte, verdict, durÃ©e enregistrÃ©s |

### 5.2 CritÃ¨res d'Ã©chec

| CritÃ¨res | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool lÃ¨ve une exception non prÃ©vue par le contrat |
| **Modification hors sandbox** | Une donnÃ©e mÃ©tier (identitÃ©, Passeport/Visa rÃ©el) est modifiÃ©e |
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

### 6.1 ExÃ©cution d'un test unitaire MiyuAuth

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. Chargement dÃ©finition du test                              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - ID du test (MAUTH-*)                                        â”‚
â”‚ - ToolId concernÃ©                                             â”‚
â”‚ - ParamÃ¨tres (sandbox, mocks, timeout, etc.)                   â”‚
â”‚ - CritÃ¨res de succÃ¨s                                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. PrÃ©paration (si sandbox / mocks)                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - CrÃ©ation sandbox / mocks (Passeport/Visa de test) si besoin â”‚
â”‚ - Via gouvernance (KindMother validation mock)                 â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. ExÃ©cution du Tool (via gouvernance)                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - BondingBrother â†’ Master Butler â†’ WorrySentinel â†’            â”‚
â”‚   Caring Nanny â†’ StrongFather â†’ MiyuAuth Tool                 â”‚
â”‚ - Collecte rÃ©sultat ou exception                              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Nettoyage (si sandbox)                                     â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Suppression donnÃ©es test / tear-down sandbox                 â”‚
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
|-------|--------------|--------------|-------|
| **Quick** | MAUTH-R-001, MAUTH-RO-001, MAUTH-RO-003 | < 1 min | VÃ©rification rapide |
| **Standard** | Tous MAUTH-R, MAUTH-RO, MAUTH-V (mocks) | 2â€“5 min | VÃ©rification quotidienne |
| **Full** | Tous MAUTH-* (avec sandbox pour attest/verify) | 5â€“10 min | VÃ©rification complÃ¨te |

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Reference Outils | [MiyuAuth - Reference Outils](../../MiyuAuth%20-%20Reference%20Outils.md) |
| MiyuAuth - Cycle Tests Contract | [MiyuAuth - Cycle Tests Contract](./MiyuAuth%20-%20Cycle%20Tests%20Contract.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

