# MiyuWeb â€” Cycle Tests Contract

## 1. Contexte

Ce document dÃ©finit le contrat pour les **tests de cycle** du kit MiyuWeb. Les tests de cycle vÃ©rifient le chemin complet (rÃ©solution thÃ¨me â†’ chargement template â†’ rendu â†’ formulaire / Ã©vÃ©nement) dans un scÃ©nario gouvernÃ© et peuvent Ãªtre exÃ©cutÃ©s par MiyukiniAdmin pour valider MiyuWeb de faÃ§on prÃ©cise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les types de tests de cycle applicables Ã  MiyuWeb (chemin complet : rÃ©solution thÃ¨me â†’ template â†’ rendu â†’ formulaire / Ã©vÃ©nement)
- Le **test chemin complet MiyuWeb** : scÃ©nario E2E (rÃ©solution thÃ¨me â†’ donnÃ©es template fournies dans le flux â†’ rendu HTML/layout â†’ validation formulaire ou dispatch/capture Ã©vÃ©nement dans un flux gouvernÃ©)
- Le lien avec MiyukiniAdmin comme exÃ©cutant du test
- Les mÃ©triques et critÃ¨res de succÃ¨s

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuWeb - Unit Tests Contract)
- L'implÃ©mentation technique des tests

---

## 3. Principe fondamental

### 3.1 Environnement de diagnostic

> **Les tests de cycle MiyuWeb sont exÃ©cutÃ©s dans un environnement de diagnostic contrÃ´lÃ©. Le test chemin complet utilise des donnÃ©es de test (templates, assets, thÃ¨me) fournies dans le flux ; aucune donnÃ©e mÃ©tier rÃ©elle n'est modifiÃ©e. MiyuWeb ne lit jamais la base directement.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MWEB-1** | Aucune modification des donnÃ©es mÃ©tier (templates ou assets de production) |
| **INV-CT-MWEB-2** | Les templates/assets de test sont fournis dans le flux ; isolation et nettoyage obligatoires si sandbox utilisÃ©e |
| **INV-CT-MWEB-3** | TraÃ§abilitÃ© complÃ¨te de chaque Ã©tape du test |
| **INV-CT-MWEB-4** | Validation StrongFather (et Cores) avant toute utilisation des Tools MiyuWeb |
| **INV-CT-MWEB-5** | Aucune lecture directe en base par MiyuWeb ; toutes les donnÃ©es (template, assets, thÃ¨me) viennent du flux |
| **INV-CT-MWEB-6** | Rapports conservÃ©s pour audit |

---

## 4. Types de tests de cycle MiyuWeb

### 4.1 Test chemin complet (thÃ¨me â†’ template â†’ rendu â†’ formulaire / Ã©vÃ©nement)

Voir section 5.

### 4.2 Tests de latence (optionnel)

| Test | Description | Cible |
|------|-------------|-------|
| **MWEB-LAT-001** | Latence tool.web.theme.resolve | < seuil configurÃ© |
| **MWEB-LAT-002** | Latence tool.web.html.render | < seuil configurÃ© |
| **MWEB-LAT-003** | Latence chemin complet (BondingBrother â†’ StrongFather â†’ MiyuWeb Tools) | < seuil configurÃ© (ex. 200 ms) |

---

## 5. Test chemin complet MiyuWeb

### 5.1 Objectif

Ce test vÃ©rifie le **chemin complet d'affichage web** : rÃ©solution du thÃ¨me, utilisation de donnÃ©es de template fournies dans le flux, rendu HTML et layout, puis validation de formulaire ou dispatch/capture d'Ã©vÃ©nement dans un scÃ©nario gouvernÃ©. MiyukiniAdmin peut exÃ©cuter ce test pour valider MiyuWeb de faÃ§on prÃ©cise.

### 5.2 DonnÃ©es de test

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Template / assets / thÃ¨me** | DonnÃ©es de test fournies dans le flux (mock ou sandbox) ; aucune donnÃ©e mÃ©tier rÃ©elle |
| **Usage** | RÃ©servÃ© Ã  ce test ; isolation et nettoyage obligatoires si sandbox utilisÃ©e |
| **Isolation** | Les donnÃ©es sont prÃ©parÃ©es, fournies dans le flux, utilisÃ©es par les Tools MiyuWeb, puis supprimÃ©es dans le cadre du test si applicable |

### 5.3 ScÃ©nario E2E (Ã©tapes)

Les Ã©tapes suivantes sont exÃ©cutÃ©es dans l'ordre. Chaque Ã©tape doit rÃ©ussir pour que le test soit considÃ©rÃ© rÃ©ussi.

| Ã‰tape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. Contexte d'entrÃ©e** | PrÃ©paration des donnÃ©es de test (thÃ¨me, template, donnÃ©es de rendu, schÃ©ma formulaire ou Ã©vÃ©nement). L'OpÃ©rateur (ou MiyukiniAdmin) prÃ©pare les donnÃ©es d'entrÃ©e et les fournit dans le flux. Aucune lecture directe en base par MiyuWeb. | OpÃ©rateur / MiyukiniAdmin â†’ BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother â†’ Master Butler â†’ WorrySentinel â†’ Caring Nanny â†’ StrongFather. La dÃ©cision doit Ãªtre ALLOW pour l'utilisation des Tools MiyuWeb. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. RÃ©solution thÃ¨me** | Appel Ã  tool.web.theme.resolve avec un contexte de test (ex. mode clair, identifiant thÃ¨me). VÃ©rification que les donnÃ©es de thÃ¨me sont retournÃ©es. | MiyuWeb (tool.web.theme.resolve) |
| **4. Rendu HTML / layout** | Appel Ã  tool.web.html.render (et optionnellement tool.web.layout.render) avec template et donnÃ©es fournis dans le flux. VÃ©rification que le HTML/layout produit est conforme. | MiyuWeb (tool.web.html.render, tool.web.layout.render) |
| **5. Formulaire ou Ã©vÃ©nement** | Soit : appel Ã  tool.web.form.validate pour un formulaire de test (structure, champs) ; soit : tool.web.event.dispatch puis tool.web.input.capture pour un scÃ©nario Ã©vÃ©nementiel. VÃ©rification que le rÃ©sultat est conforme. | MiyuWeb (tool.web.form.validate ou tool.web.event.dispatch / tool.web.input.capture) |
| **6. Nettoyage** | Suppression des donnÃ©es de test si sandbox utilisÃ©e ; tear-down documentÃ©. | Via gouvernance |

### 5.4 CritÃ¨res de succÃ¨s du test

| CritÃ¨re | Description |
|---------|-------------|
| **C1** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) exÃ©cutÃ©es et ALLOW obtenu |
| **C2** | tool.web.theme.resolve retourne des donnÃ©es de thÃ¨me cohÃ©rentes pour le contexte fourni |
| **C3** | tool.web.html.render (et tool.web.layout.render si applicable) retourne du HTML/layout conforme aux entrÃ©es du flux |
| **C4** | tool.web.form.validate (ou tool.web.event.dispatch / tool.web.input.capture) retourne un rÃ©sultat conforme |
| **C5** | Aucune lecture directe en base par MiyuWeb ; toutes les entrÃ©es proviennent du flux |
| **C6** | Nettoyage effectuÃ© ; aucune donnÃ©e mÃ©tier modifiÃ©e |

### 5.5 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les Ã©tapes 1 Ã  6 rÃ©ussies et tous les critÃ¨res C1â€“C6 remplis |
| **FAIL** | Une Ã©tape Ã©choue ou un critÃ¨re n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.6 ExÃ©cutant : MiyukiniAdmin

MiyukiniAdmin peut exÃ©cuter ce test pour vÃ©rifier le chemin complet MiyuWeb. La rÃ©fÃ©rence croisÃ©e peut Ãªtre Ã©tablie dans MiyukiniAdmin - Cycle Tests Contract (section MiyuWeb Full Path Test), qui pointe vers ce contrat pour la spÃ©cification du scÃ©nario et des critÃ¨res.

---

## 6. Protocole d'exÃ©cution

### 6.1 Phases

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 1 : PrÃ©paration                                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - VÃ©rification prÃ©-conditions (gouvernance, Ã©tat systÃ¨me)    â”‚
â”‚ - PrÃ©paration donnÃ©es de test (thÃ¨me, template, formulaire   â”‚
â”‚   ou Ã©vÃ©nement) Ã  fournir dans le flux ; pas de lecture base â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 2 : ExÃ©cution (Ã©tapes 1 Ã  6)                           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Contexte â†’ Validations Cores â†’ theme.resolve â†’            â”‚
â”‚   html.render / layout.render â†’ form.validate ou             â”‚
â”‚   event.dispatch / input.capture â†’ Nettoyage                 â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 3 : Rapport                                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Verdict (PASS/FAIL/ERROR)                                 â”‚
â”‚ - DÃ©tails par Ã©tape                                          â”‚
â”‚ - Stockage pour audit                                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 7. MÃ©triques et rapports

### 7.1 MÃ©triques collectÃ©es (optionnel)

| MÃ©trique | Description |
|----------|-------------|
| `step_duration_ms` | DurÃ©e par Ã©tape (1 Ã  6) |
| `total_duration_ms` | DurÃ©e totale du test |
| `validation_latency_ms` | Latence cumulÃ©e des validations Cores |
| `render_latency_ms` | Latence du rendu HTML/layout |

### 7.2 Structure rapport (rÃ©sumÃ©)

```json
{
  "test_id": "MiyuWeb_FullPath",
  "timestamp": "2026-01-30T12:00:00Z",
  "verdict": "PASS",
  "steps": [
    {"step": 1, "name": "Contexte entree", "status": "OK"},
    {"step": 2, "name": "Validations Cores", "status": "OK"},
    {"step": 3, "name": "Resolution theme", "status": "OK"},
    {"step": 4, "name": "Rendu HTML / layout", "status": "OK"},
    {"step": 5, "name": "Formulaire ou evenement", "status": "OK"},
    {"step": 6, "name": "Nettoyage", "status": "OK"}
  ],
  "criteria_met": true,
  "duration_ms": 450
}
```

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuWeb - Documentation Fondatrice | [MiyuWeb - Documentation Fondatrice](../../MiyuWeb%20-%20Documentation%20Fondatrice.md) |
| MiyuWeb - Unit Tests Contract | [MiyuWeb - Unit Tests Contract](./MiyuWeb%20-%20Unit%20Tests%20Contract.md) |
| MiyuWeb - KindMother Integration Contract | [MiyuWeb - KindMother Integration Contract](../integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) |
| MiyuWeb - Runtime Boundary Contract | [MiyuWeb - Runtime Boundary Contract](../boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

