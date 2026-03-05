# MiyuAuth â€” Cycle Tests Contract

## 1. Contexte

Ce document dÃ©finit le contrat pour les **tests de cycle** du kit MiyuAuth. Les tests de cycle vÃ©rifient le chemin complet (rÃ©solution identitÃ© â†’ rÃ´le â†’ vÃ©rification Passeport/Visa) dans un scÃ©nario gouvernÃ© et peuvent Ãªtre exÃ©cutÃ©s par MiyukiniAdmin pour valider MiyuAuth de faÃ§on prÃ©cise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- Les types de tests de cycle applicables Ã  MiyuAuth (chemin complet, rÃ©solution â†’ rÃ´le â†’ vÃ©rification)
- Le **test chemin complet MiyuAuth** : scÃ©nario E2E (rÃ©solution identitÃ© â†’ rÃ´le â†’ vÃ©rification Passeport/Visa dans un flux gouvernÃ©)
- Le lien avec MiyukiniAdmin comme exÃ©cutant du test
- Les mÃ©triques et critÃ¨res de succÃ¨s

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuAuth - Unit Tests Contract)
- L'implÃ©mentation technique des tests

---

## 3. Principe fondamental

### 3.1 Environnement de diagnostic

> **Les tests de cycle MiyuAuth sont exÃ©cutÃ©s dans un environnement de diagnostic contrÃ´lÃ©. Le test chemin complet utilise des donnÃ©es de test (Passeport/Visa de test) ; aucune donnÃ©e mÃ©tier rÃ©elle n'est modifiÃ©e.**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MAUTH-1** | Aucune modification des donnÃ©es mÃ©tier (identitÃ©s de production, Passeports/Visas rÃ©els) |
| **INV-CT-MAUTH-2** | Les Passeports/Visas de test sont rÃ©servÃ©s au test ; isolation et nettoyage obligatoires |
| **INV-CT-MAUTH-3** | TraÃ§abilitÃ© complÃ¨te de chaque Ã©tape du test |
| **INV-CT-MAUTH-4** | Validation StrongFather (et Cores) avant toute utilisation des Tools MiyuAuth |
| **INV-CT-MAUTH-5** | Validation KindMother (confiance) pour toute utilisation de confiance inter-domaines dans le scÃ©nario |
| **INV-CT-MAUTH-6** | Rapports conservÃ©s pour audit |

---

## 4. Types de tests de cycle MiyuAuth

### 4.1 Test chemin complet (rÃ©solution â†’ rÃ´le â†’ vÃ©rification)

Voir section 5.

### 4.2 Tests de latence (optionnel)

| Test | Description | Cible |
|------|-------------|-------|
| **MAUTH-LAT-001** | Latence tool.identity.resolve | < seuil configurÃ© |
| **MAUTH-LAT-002** | Latence tool.identity.verify | < seuil configurÃ© |
| **MAUTH-LAT-003** | Latence chemin complet (BondingBrother â†’ KindMother â†’ MiyuAuth) | < 100 ms |

---

## 5. Test chemin complet MiyuAuth

### 5.1 Objectif

Ce test vÃ©rifie le **chemin complet d'identitÃ©** : rÃ©solution d'un contexte d'identitÃ© (citoyen / visiteur / externe), rÃ©solution du rÃ´le, et vÃ©rification d'un Passeport Utilisateur ou Visa de Connexion dans un scÃ©nario gouvernÃ©. MiyukiniAdmin peut exÃ©cuter ce test pour valider MiyuAuth de faÃ§on prÃ©cise.

### 5.2 DonnÃ©es de test

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Passeport / Visa** | DonnÃ©es de test (mock ou sandbox) ; aucune donnÃ©e mÃ©tier rÃ©elle |
| **Usage** | RÃ©servÃ© Ã  ce test ; isolation et nettoyage obligatoires |
| **Isolation** | Les donnÃ©es sont crÃ©Ã©es, utilisÃ©es et supprimÃ©es dans le cadre du test |

### 5.3 ScÃ©nario E2E (Ã©tapes)

Les Ã©tapes suivantes sont exÃ©cutÃ©es dans l'ordre. Chaque Ã©tape doit rÃ©ussir pour que le test soit considÃ©rÃ© rÃ©ussi.

| Ã‰tape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. Contexte d'entrÃ©e** | PrÃ©paration d'un contexte d'identitÃ© de test (citoyen, visiteur ou externe). L'OpÃ©rateur (ou MiyukiniAdmin) prÃ©pare les donnÃ©es d'entrÃ©e (Passeport/Visa de test ou absence de certificat). | OpÃ©rateur / MiyukiniAdmin â†’ BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother â†’ Master Butler â†’ WorrySentinel â†’ Caring Nanny â†’ StrongFather. La dÃ©cision doit Ãªtre ALLOW pour l'utilisation des Tools MiyuAuth. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. Validation KindMother** | Pour toute utilisation de confiance inter-domaines (ex. visiteur), KindMother valide la confiance. | KindMother |
| **4. RÃ©solution identitÃ©** | Appel Ã  tool.identity.resolve avec le contexte de test. VÃ©rification que le contexte est rÃ©solu (citoyen / visiteur / externe). | MiyuAuth (tool.identity.resolve) |
| **5. RÃ´le rÃ©solu** | Appel Ã  tool.identity.role pour le contexte gouvernÃ©. VÃ©rification que le rÃ´le retournÃ© est cohÃ©rent (citoyen, visiteur, externe). | MiyuAuth (tool.identity.role) |
| **6. VÃ©rification Passeport/Visa** | Si contexte visiteur : appel Ã  tool.identity.verify avec Passeport/Visa de test. VÃ©rification que la structure et la signature sont reconnues (vÃ©rification technique ; confiance validÃ©e par KindMother). | MiyuAuth (tool.identity.verify) |
| **7. Attestation (optionnel)** | Si applicable : appel Ã  tool.identity.attest pour un contexte validÃ© par KindMother. VÃ©rification que l'attestation est produite. | MiyuAuth (tool.identity.attest) |
| **8. Nettoyage** | Suppression des donnÃ©es de test ; tear-down documentÃ©. | Via gouvernance |

### 5.4 CritÃ¨res de succÃ¨s du test

| CritÃ¨re | Description |
|---------|-------------|
| **C1** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) exÃ©cutÃ©es et ALLOW obtenu |
| **C2** | Validation KindMother (confiance) obtenue pour toute utilisation de confiance inter-domaines |
| **C3** | tool.identity.resolve retourne un contexte cohÃ©rent (citoyen / visiteur / externe) |
| **C4** | tool.identity.role retourne le rÃ´le attendu |
| **C5** | tool.identity.verify (si applicable) retourne un verdict conforme (structure, signature) |
| **C6** | tool.identity.attest (si applicable) produit une attestation pour un contexte validÃ© |
| **C7** | Nettoyage effectuÃ© ; aucune donnÃ©e mÃ©tier modifiÃ©e |

### 5.5 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les Ã©tapes 1 Ã  8 rÃ©ussies et tous les critÃ¨res C1â€“C7 remplis |
| **FAIL** | Une Ã©tape Ã©choue ou un critÃ¨re n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.6 ExÃ©cutant : MiyukiniAdmin

MiyukiniAdmin peut exÃ©cuter ce test pour vÃ©rifier le chemin complet MiyuAuth. La rÃ©fÃ©rence croisÃ©e peut Ãªtre Ã©tablie dans MiyukiniAdmin - Cycle Tests Contract (section MiyuAuth Full Path Test), qui pointe vers ce contrat pour la spÃ©cification du scÃ©nario et des critÃ¨res.

---

## 6. Protocole d'exÃ©cution

### 6.1 Phases

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 1 : PrÃ©paration                                        â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - VÃ©rification prÃ©-conditions (gouvernance, Ã©tat systÃ¨me)    â”‚
â”‚ - CrÃ©ation donnÃ©es de test (Passeport/Visa mock) si besoin   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                              â”‚
                              â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Phase 2 : ExÃ©cution (Ã©tapes 1 Ã  8)                           â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ - Contexte â†’ Validations Cores â†’ KindMother â†’ resolve       â”‚
â”‚ - role â†’ verify (si visiteur) â†’ attest (optionnel) â†’ Nettoyageâ”‚
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
| `step_duration_ms` | DurÃ©e par Ã©tape (1 Ã  8) |
| `total_duration_ms` | DurÃ©e totale du test |
| `validation_latency_ms` | Latence cumulÃ©e des validations Cores et KindMother |

### 7.2 Structure rapport (rÃ©sumÃ©)

```json
{
  "test_id": "MiyuAuth_FullPath",
  "timestamp": "2026-01-30T12:00:00Z",
  "verdict": "PASS",
  "steps": [
    {"step": 1, "name": "Contexte entree", "status": "OK"},
    {"step": 2, "name": "Validations Cores", "status": "OK"},
    {"step": 3, "name": "Validation KindMother", "status": "OK"},
    {"step": 4, "name": "Resolution identite", "status": "OK"},
    {"step": 5, "name": "Role resolu", "status": "OK"},
    {"step": 6, "name": "Verification Passeport/Visa", "status": "OK"},
    {"step": 7, "name": "Attestation", "status": "OK"},
    {"step": 8, "name": "Nettoyage", "status": "OK"}
  ],
  "criteria_met": true,
  "duration_ms": 350
}
```

---

## 8. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuAuth - Documentation Fondatrice | [MiyuAuth - Documentation Fondatrice](../../MiyuAuth%20-%20Documentation%20Fondatrice.md) |
| MiyuAuth - Unit Tests Contract | [MiyuAuth - Unit Tests Contract](./MiyuAuth%20-%20Unit%20Tests%20Contract.md) |
| MiyuAuth - KindMother Integration Contract | [MiyuAuth - KindMother Integration Contract](../integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](..//..//..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de rÃ©fÃ©rence

