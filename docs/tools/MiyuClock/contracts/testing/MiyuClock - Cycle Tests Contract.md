# MiyuClock — Cycle Tests Contract

## 1. Contexte

Ce document définit le contrat pour les **tests de cycle** du kit MiyuClock. Les tests de cycle vérifient le chemin complet (obtention de l'instant présent → calcul du delta → utilisation dans le flux) dans un scénario gouverné et peuvent être exécutés par MiyukiniAdmin pour valider MiyuClock de façon précise.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Les types de tests de cycle applicables à MiyuClock (chemin complet : now → delta → utilisation dans le flux)
- Le **test chemin complet MiyuClock** : scénario E2E (opérateur → now → delta → utilisation dans le flux gouverné)
- Le lien avec MiyukiniAdmin comme exécutant du test
- Les métriques et critères de succès

Ce document **ne couvre pas** :
- Les tests unitaires (voir MiyuClock - Unit Tests Contract)
- L'implémentation technique des tests
- La persistance des timestamps (Opérateur + KindMother/MiyuSQL)

---

## 3. Principe fondamental

### 3.1 Environnement de diagnostic

> **Les tests de cycle MiyuClock sont exécutés dans un environnement de diagnostic contrôlé. Le test chemin complet utilise des valeurs de temps produites dans le flux ; aucune persistance par MiyuClock. Aucune dépendance à un temps global (LOI-4).**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-CT-MCLOCK-1** | Aucune persistance par MiyuClock ; les valeurs de temps restent dans le flux de test |
| **INV-CT-MCLOCK-2** | Aucune dépendance à un temps global ; horloge locale (Kernel Clock) uniquement |
| **INV-CT-MCLOCK-3** | Traçabilité complète de chaque étape du test |
| **INV-CT-MCLOCK-4** | Validation StrongFather (et Cores) avant toute utilisation des Tools MiyuClock |
| **INV-CT-MCLOCK-5** | Rapports conservés pour audit |

---

## 4. Types de tests de cycle MiyuClock

### 4.1 Test chemin complet (now → delta → utilisation dans le flux)

Voir section 5.

### 4.2 Tests de latence (optionnel)

| Test | Description | Cible |
|------|-------------|-------|
| **MCLOCK-LAT-001** | Latence tool.time.now | < seuil configuré |
| **MCLOCK-LAT-002** | Latence tool.time.delta | < seuil configuré |
| **MCLOCK-LAT-003** | Latence chemin complet (BondingBrother → MiyuClock now → delta) | < seuil configuré |

---

## 5. Test chemin complet MiyuClock

### 5.1 Objectif

Ce test vérifie le **chemin complet de mesure du temps** : obtention de l'instant présent via `tool.time.now`, calcul de la durée écoulée via `tool.time.delta` entre deux instants, et utilisation du résultat dans le flux (ex. décision métier par l'Opérateur, passage à KindMother pour persistance si l'Opérateur le décide). MiyukiniAdmin peut exécuter ce test pour valider MiyuClock de façon précise.

### 5.2 Données de test

| Élément | Valeur |
|---------|--------|
| **Instants** | Produits par `tool.time.now` dans le flux gouverné ; aucune lecture en base |
| **Usage** | Réservé au flux de test ; aucune persistance par MiyuClock |
| **Isolation** | Les valeurs sont produites, utilisées dans le flux et éventuellement persistées par l'Opérateur + KindMother (hors périmètre MiyuClock) |

### 5.3 Scénario E2E (étapes)

Les étapes suivantes sont exécutées dans l'ordre. Chaque étape doit réussir pour que le test soit considéré réussi.

| Étape | Description | Acteurs / Tools |
|-------|-------------|-----------------|
| **1. Contexte d'entrée** | L'Opérateur (ou MiyukiniAdmin) prépare la demande d'utilisation du kit MiyuClock (now puis delta). Aucune donnée métier persistée par MiyuClock. | Opérateur / MiyukiniAdmin → BondingBrother |
| **2. Validations Cores** | Parcours explicite : BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. La décision doit être ALLOW pour l'utilisation des Tools MiyuClock. | BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather |
| **3. Obtention instant initial (t_prev)** | Appel à `tool.time.now` via la gouvernance. Vérification que l'instant retourné est du type attendu (référence locale). Stockage dans le flux pour l'étape suivante. | MiyuClock (tool.time.now) |
| **4. Attente ou traitement (optionnel)** | Si le scénario le requiert : courte attente ou traitement simulé pour obtenir un t_now distinct de t_prev. | Flux de test |
| **5. Obtention instant final (t_now)** | Appel à `tool.time.now` via la gouvernance. Vérification que l'instant retourné est du type attendu. | MiyuClock (tool.time.now) |
| **6. Calcul du delta** | Appel à `tool.time.delta(t_prev, t_now)` via la gouvernance. Vérification que la durée retournée est conforme (positive lorsque t_now > t_prev). | MiyuClock (tool.time.delta) |
| **7. Utilisation dans le flux** | Le résultat (durée) est utilisé dans le flux : vérification que la valeur est exploitable (ex. comparaison à un seuil, passage à l'Opérateur pour décision). Si persistance : Opérateur + KindMother (hors périmètre MiyuClock). | Flux / Opérateur |
| **8. Nettoyage** | Aucune persistance par MiyuClock à nettoyer ; tear-down du flux de test documenté. | Via gouvernance |

### 5.4 Critères de succès du test

| Critère | Description |
|---------|-------------|
| **C1** | Validations Cores (BondingBrother, Master Butler, WorrySentinel, Caring Nanny, StrongFather) exécutées et ALLOW obtenu |
| **C2** | `tool.time.now` retourne un instant de type attendu (référence locale) à chaque appel |
| **C3** | `tool.time.delta(t_prev, t_now)` retourne une durée conforme (positive lorsque t_now > t_prev) |
| **C4** | La durée calculée est exploitable dans le flux (ex. cohérente avec l'écart entre t_prev et t_now) |
| **C5** | Aucune persistance par MiyuClock ; aucune dépendance temps global (LOI-4) |
| **C6** | Nettoyage du flux de test effectué ; aucune violation des contrats MiyuClock |

### 5.5 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Toutes les étapes 1 à 8 réussies et tous les critères C1–C6 remplis |
| **FAIL** | Une étape échoue ou un critère n'est pas rempli |
| **ERROR** | Erreur technique (environnement, configuration, gouvernance indisponible) |

### 5.6 Exécutant : MiyukiniAdmin

MiyukiniAdmin peut exécuter ce test pour vérifier le chemin complet MiyuClock. La référence croisée peut être établie dans MiyukiniAdmin - Cycle Tests Contract (section MiyuClock Full Path Test), qui pointe vers ce contrat pour la spécification du scénario et des critères.

---

## 6. Protocole d'exécution

### 6.1 Phases

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1 : Préparation                                        │
├─────────────────────────────────────────────────────────────┤
│ - Vérification pré-conditions (gouvernance, état système)     │
│ - Aucune donnée de test à créer (MiyuClock ne persiste pas) │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 2 : Exécution (étapes 1 à 8)                           │
├─────────────────────────────────────────────────────────────┤
│ - Contexte → Validations Cores → now (t_prev) → now (t_now)  │
│ - delta(t_prev, t_now) → utilisation dans le flux → Nettoyage │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ Phase 3 : Rapport                                            │
├─────────────────────────────────────────────────────────────┤
│ - Verdict (PASS/FAIL/ERROR)                                  │
│ - Détails par étape                                          │
│ - Stockage pour audit                                        │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Métriques et rapports

### 7.1 Métriques collectées (optionnel)

| Métrique | Description |
|----------|-------------|
| `step_duration_ms` | Durée par étape (1 à 8) |
| `total_duration_ms` | Durée totale du test |
| `validation_latency_ms` | Latence cumulée des validations Cores |
| `now_latency_ms` | Latence de tool.time.now |
| `delta_latency_ms` | Latence de tool.time.delta |

### 7.2 Structure rapport (résumé)

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

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Unit Tests Contract | [MiyuClock - Unit Tests Contract](./MiyuClock%20-%20Unit%20Tests%20Contract.md) |
| MiyuClock - KindMother Integration Contract | [MiyuClock - KindMother Integration Contract](../integration/MiyuClock%20-%20KindMother%20Integration%20Contract.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
