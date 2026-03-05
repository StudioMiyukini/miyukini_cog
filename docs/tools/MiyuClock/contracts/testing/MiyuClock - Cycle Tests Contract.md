# MiyuClock â€” Cycle Tests Contract

## 1. Contexte

Ce document dÃ©finit le contrat pour les **tests de cycle** du kit MiyuClock. Les tests de cycle vÃ©rifient le chemin complet (obtention de l'instant prÃ©sent â†’ calcul du delta â†’ utilisation dans le flux) dans un scÃ©nario gouvernÃ© et peuvent Ãªtre exÃ©cutÃ©s par MiyukiniAdmin pour valider MiyuClock de faÃ§on prÃ©cise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les types de tests de cycle applicables Ã  MiyuClock (chemin complet : now â†’ delta â†’ utilisation dans le flux)
- Le **test chemin complet MiyuClock** : scÃ©nario E2E (opÃ©rateur â†’ now â†’ delta â†’ utilisation dans le flux gouvernÃ©)
- Le lien avec MiyukiniAdmin comme exÃ©cutant du test
- Les mÃ©triques et critÃ¨res de succÃ¨s

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuClock - Unit Tests Contract)
- L'implÃ©mentation technique des tests
- La persistance des timestamps (OpÃ©rateur + KindMother/MiyuSQL)

---

## 3. Principe fondamental

### 3.1 Environnement de diagnostic

> **Les tests de cycle MiyuClock sont exÃ©cutÃ©s dans un environnement de diagnostic contrÃ´lÃ©. Le test chemin complet utilise des valeurs de temps produites dans le flux ; aucune persistance par MiyuClock. Aucune dÃ©pendance Ã  un temps global (LOI-4).**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MCLOCK-1** | Aucune persistance par MiyuClock ; les valeurs de temps restent dans le flux de test |
| **INV-CT-MCLOCK-2** | Aucune dÃ©pendance Ã  un temps global ; horloge locale (Kernel Clock) uniquement |
| **INV-CT-MCLOCK-3** | TraÃ§abilitÃ© complÃ¨te de chaque Ã©tape du test |
| **INV-CT-MCLOCK-4** | Validation StrongFather (et Cores) avant toute utilisation des Tools MiyuClock |
| **INV-CT-MCLOCK-5** | Rapports conservÃ©s pour audit |

---

## 4. Types de tests de cycle MiyuClock

### 4.1 Test chemin complet (now â†’ delta â†’ utilisation dans le flux)

Voir section 5.

### 4.2 Tests de latence (optionnel)

| Test | Description | Cible |
|------|-------------|-------|
| **MCLOCK-LAT-001** | Latence tool.time.now | < seuil configurÃ© |
| **MCLOCK-LAT-002** | Latence tool.time.delta | < seuil configurÃ© |
| **MCLOCK-LAT-003** | Latence chemin complet (BondingBrother â†’ MiyuClock now â†’ delta) | < seuil configurÃ© |

---

## 5. Test chemin complet MiyuClock

### 5.1 Objectif

Ce test vÃ©rifie le **chemin complet de mesure du temps** : obtention de l'instant prÃ©sent via `tool.time.now`, calcul de la durÃ©e Ã©coulÃ©e via `tool.time.delta` entre deux instants, et utilisation du rÃ©sultat dans le flux (ex. dÃ©cision mÃ©tier par l'OpÃ©rateur, passage Ã  KindMother pour persistance si l'OpÃ©rateur le dÃ©cide). MiyukiniAdmin peut exÃ©cuter ce test pour valider MiyuClock de faÃ§on prÃ©cise.

### 5.2 DonnÃ©es de test

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Instants** | Produits par `tool.time.now` dans le flux gouvernÃ© ; aucune lecture en base |
| **Usage** | RÃ©servÃ© au flux de test ; aucune persistance par MiyuClock |
| **Isolation** | Les valeurs sont produites, utilisÃ©es dans le flux et Ã©ventuellement persistÃ©es par l'OpÃ©rateur + KindMother (hors pÃ©rimÃ¨tre MiyuClock) |

### 5.3 ScÃ©nario E2E (Ã©tapes)

Les Ã©tapes suivantes sont exÃ©cutÃ©es dans l'ordre. Chaque Ã©tape doit rÃ©ussir pour que le test soit considÃ©rÃ© rÃ©ussi.

| Ã‰tape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. Contexte d'entrÃ©e** | L'OpÃ©rateur (ou MiyukiniAdmin) prÃ©pare la demande d'utilisation du kit MiyuClock (now puis delta). Aucune donnÃ©e mÃ©tier persistÃ©e par MiyuClock. | OpÃ©rateur / MiyukiniAdmin â†’ BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother â†’ Master Butler â†’ WorrySentinel â†’ Caring Nanny â†’ StrongFather. La dÃ©cision doit Ãªtre ALLOW pour l'utilisation des Tools MiyuClock. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. Obtention instant initial (t_prev)** | Appel Ã  `tool.time.now` via la gouvernance. VÃ©rification que l'instant retournÃ© est du type attendu (rÃ©fÃ©rence locale). Stockage dans le flux pour l'Ã©tape suivante. | MiyuClock (tool.time.now) |
| **4. Attente ou traitement (optionnel)** | Si le scÃ©nario le requiert : courte attente ou traitement simulÃ© pour obtenir un t_now distinct de t_prev. | Flux de test |
| **5. Obtention instant final (t_now)** | Appel Ã  `tool.time.now` via la gouvernance. VÃ©rification que l'instant retournÃ© est du type attendu. | MiyuClock (tool.time.now) |
| **6. Calcul du delta** | Appel Ã  `tool.time.delta(t_prev, t_now)` via la gouvernance. VÃ©rification que la durÃ©e retournÃ©e est conforme (positive lorsque t_now > t_prev). | MiyuClock (tool.time.delta) |
| **7. Utilisation dans le flux** | Le rÃ©sultat (durÃ©e) est utilisÃ© dans le flux : vÃ©rification que la valeur est exploitable (ex. comparaison Ã  un seuil, passage Ã  l'OpÃ©rateur pour dÃ©cision). Si persistance : OpÃ©rateur + KindMother (hors pÃ©rimÃ¨tre MiyuClock). | Flux / OpÃ©rateur |
| **8. Nettoyage** | Aucune persistance par MiyuClock Ã  nettoyer ; tear-down du flux de test documentÃ©. | Via gouvernance |

### 5.4 CritÃ¨res de succÃ¨s du test

| CritÃ¨re | Description |
|---------|-------------|
| **C1** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) exÃ©cutÃ©es et ALLOW obtenu |
| **C2** | `tool.time.now` retourne un instant de type attendu (rÃ©fÃ©rence locale) Ã  chaque appel |
| **C3** | `tool.time.delta(t_prev, t_now)` retourne une durÃ©e conforme (positive lorsque t_now > t_prev) |
| **C4** | La durÃ©e calculÃ©e est exploitable dans le flux (ex. cohÃ©rente avec l'Ã©cart entre t_prev et t_now) |
| **C5** | Aucune persistance par MiyuClock ; aucune dÃ©pendance temps global (LOI-4) |
| **C6** | Nettoyage du flux de test effectuÃ© ; aucune violation des contrats MiyuClock |

### 5.5 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les Ã©tapes 1 Ã  8 rÃ©ussies et tous les critÃ¨res C1â€“C6 remplis |
| **FAIL** | Une Ã©tape Ã©choue ou un critÃ¨re n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.6 ExÃ©cutant : MiyukiniAdmin

MiyukiniAdmin peut exÃ©cuter ce test pour vÃ©rifier le chemin complet MiyuClock. La rÃ©fÃ©rence croisÃ©e peut Ãªtre Ã©tablie dans MiyukiniAdmin - Cycle Tests Contract (section MiyuClock Full Path Test), qui pointe vers ce contrat pour la spÃ©cification du scÃ©nario et des critÃ¨res.

---

## 6. Protocole d'exÃ©cution

### 6.1 Phases

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 1 : PrÃ©paration                                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - VÃ©rification prÃ©-conditions (gouvernance, Ã©tat systÃ¨me)     â”‚
â”‚ - Aucune donnÃ©e de test Ã  crÃ©er (MiyuClock ne persiste pas) â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 2 : ExÃ©cution (Ã©tapes 1 Ã  8)                           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Contexte â†’ Validations Cores â†’ now (t_prev) â†’ now (t_now)  â”‚
â”‚ - delta(t_prev, t_now) â†’ utilisation dans le flux â†’ Nettoyage â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 3 : Rapport                                            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Verdict (PASS/FAIL/ERROR)                                  â”‚
â”‚ - DÃ©tails par Ã©tape                                          â”‚
â”‚ - Stockage pour audit                                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 7. MÃ©triques et rapports

### 7.1 MÃ©triques collectÃ©es (optionnel)

| MÃ©trique | Description |
|----------|-------------|
| `step_duration_ms` | DurÃ©e par Ã©tape (1 Ã  8) |
| `total_duration_ms` | DurÃ©e totale du test |
| `validation_latency_ms` | Latence cumulÃ©e des validations Cores |
| `now_latency_ms` | Latence de tool.time.now |
| `delta_latency_ms` | Latence de tool.time.delta |

### 7.2 Structure rapport (rÃ©sumÃ©)

```json
{
  "test_id": "MiyuClock_FullPath",
  "timestamp": "2026-01-30T12:00:00Z",
  "verdict": "PASS",
  "steps": [
    {"step": 1, "name": "Contexte entree", "status": "OK"},
    {"step": 2, "name": "Validations Cores", "status": "OK"},
    {"step": 3, "name": "Obtention t_prev (now)", "status": "OK"},
    {"step": 4, "name": "Attente ou traitement", "status": "OK"},
    {"step": 5, "name": "Obtention t_now (now)", "status": "OK"},
    {"step": 6, "name": "Calcul delta", "status": "OK"},
    {"step": 7, "name": "Utilisation dans le flux", "status": "OK"},
    {"step": 8, "name": "Nettoyage", "status": "OK"}
  ],
  "criteria_met": true,
  "duration_ms": 120
}
```

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Unit Tests Contract | [MiyuClock - Unit Tests Contract](./MiyuClock%20-%20Unit%20Tests%20Contract.md) |
| MiyuClock - KindMother Integration Contract | [MiyuClock - KindMother Integration Contract](../integration/MiyuClock%20-%20KindMother%20Integration%20Contract.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

