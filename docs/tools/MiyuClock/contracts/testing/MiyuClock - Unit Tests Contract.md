# MiyuClock — Unit Tests Contract

## 1. Contexte

Ce document définit le contrat pour les **tests unitaires** des Tools du kit MiyuClock. Les tests unitaires vérifient le comportement de chaque Tool (now, delta) sans persistance, sans décision métier et sans dépendance à un temps global (conformité **LOI-4**).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

Ce document définit :
- Les types de tests unitaires sur les Tools MiyuClock (tool.time.now, tool.time.delta)
- Les critères de succès et d'échec
- La non-destructivité et l'absence de persistance
- Les protocoles de vérification (comportement de now : type de sortie, local ; delta : t_prev, t_now → durée ; pas de dépendance temps global)

Ce document **ne couvre pas** :
- Les tests de cycle (voir MiyuClock - Cycle Tests Contract)
- L'implémentation technique des tests
- La persistance des timestamps (Opérateur + KindMother/MiyuSQL)

---

## 3. Principe fondamental

### 3.1 Non-destructivité et absence de persistance

> **Les tests unitaires MiyuClock vérifient le comportement des Tools sans persister de données. Les valeurs de temps sont produites et consommées dans le flux de test ; aucune écriture en base. Aucune dépendance à un temps global (LOI-4).**

### 3.2 Invariants

| Code | Invariant |
|------|-----------|
| **INV-UT-MCLOCK-1** | Aucune persistance ; les tests n'écrivent pas de timestamps en base |
| **INV-UT-MCLOCK-2** | Aucune dépendance à un temps global (NTP, serveur externe) ; horloge locale uniquement |
| **INV-UT-MCLOCK-3** | Traçabilité complète de chaque test (contexte, verdict, durée) |
| **INV-UT-MCLOCK-4** | Rapports conservés pour audit |

---

## 4. Catégories de tests par Tool

### 4.1 Tests Instant présent (tool.time.now)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MCLOCK-N-001** | Type de sortie | Vérifie que la sortie de `tool.time.now` est du type attendu (instant / horodatage local) | Aucune ; appel gouverné |
| **MCLOCK-N-002** | Référence locale | Vérifie que l'instant retourné provient de l'horloge locale (Kernel Clock), pas d'une source externe | Aucune |
| **MCLOCK-N-003** | Pas de timezone imposée | Vérifie que MiyuClock n'impose pas de timezone ; la valeur est une référence locale | Aucune |
| **MCLOCK-N-004** | Pas de temps global | Vérifie l'absence de dépendance à un temps global (LOI-4) ; le test ne requiert pas NTP ni serveur de temps | Aucune |

### 4.2 Tests Delta entre instants (tool.time.delta)

| Code | Test | Description | Données |
|------|------|-------------|---------|
| **MCLOCK-D-001** | Delta positif | Vérifie que `tool.time.delta(t_prev, t_now)` retourne une durée positive lorsque t_now > t_prev | t_prev, t_now fournis dans le flux (types compatibles avec sortie de now) |
| **MCLOCK-D-002** | Delta nul | Vérifie que `tool.time.delta(t, t)` retourne une durée nulle (ou équivalente) | t fourni dans le flux |
| **MCLOCK-D-003** | Ordre des arguments | Vérifie le comportement documenté lorsque t_prev > t_now (durée négative ou valeur absolue selon contrat d'implémentation) | t_prev, t_now |
| **MCLOCK-D-004** | Types compatibles | Vérifie que les entrées (références ou sorties de `tool.time.now`) sont acceptées et que la sortie est une durée | Instants fournis dans le flux |
| **MCLOCK-D-005** | Pas de temps global | Vérifie que le calcul de delta ne dépend pas d'un temps global ; uniquement les instants fournis dans le flux | t_prev, t_now |

---

## 5. Critères de succès et d'échec

### 5.1 Critères de succès

| Critères | Description |
|----------|-------------|
| **Exécution conforme** | Le Tool s'exécute comme spécifié (pas d'exception non contractuelle) |
| **Résultat attendu** | Pour now : type d'instant local conforme ; pour delta : durée conforme aux entrées |
| **Pas de persistance** | Aucune écriture en base ; valeurs dans le flux uniquement |
| **Pas de temps global** | Aucune dépendance à NTP ou serveur de temps externe |
| **Traçabilité** | Contexte, verdict, durée enregistrés |

### 5.2 Critères d'échec

| Critères | Description |
|----------|-------------|
| **Exception non contractuelle** | Le Tool lève une exception non prévue par le contrat |
| **Persistance effectuée** | Une écriture en base est effectuée par MiyuClock pendant le test |
| **Dépendance temps global** | Le test ou le Tool requiert un temps global (NTP, etc.) |
| **Type de sortie incorrect** | now ne retourne pas un type d'instant attendu ; delta ne retourne pas une durée |
| **Timeout dépassé** | Le test dépasse le timeout configuré sans résultat |

### 5.3 Verdicts

| Verdict | Description |
|---------|-------------|
| **PASS** | Tous les critères de succès sont remplis |
| **WARN** | Comportement conforme avec alertes mineures (ex. performance) |
| **FAIL** | Un ou plusieurs critères d'échec sont remplis |
| **SKIP** | Pré-condition non remplie (ex. Kernel Clock indisponible) |
| **ERROR** | Erreur technique pendant le test (configuration, environnement) |

---

## 6. Protocole de test

### 6.1 Exécution d'un test unitaire MiyuClock

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Chargement définition du test                              │
├─────────────────────────────────────────────────────────────┤
│ - ID du test (MCLOCK-*)                                       │
│ - ToolId concerné (tool.time.now ou tool.time.delta)          │
│ - Paramètres (instants pour delta, timeout, etc.)             │
│ - Critères de succès                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Préparation (si nécessaire pour delta)                     │
├─────────────────────────────────────────────────────────────┤
│ - Obtenir t_prev, t_now via tool.time.now (gouverné) ou       │
│   fournir des instants de test compatibles                    │
│ - Aucune persistance                                          │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Exécution du Tool (via gouvernance)                         │
├─────────────────────────────────────────────────────────────┤
│ - BondingBrother → Master Butler → WorrySentinel →            │
│   Caring Nanny → StrongFather → MiyuClock Tool                │
│ - Collecte résultat ou exception                              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Évaluation et rapport                                      │
├─────────────────────────────────────────────────────────────┤
│ - Verdict (PASS/WARN/FAIL/SKIP/ERROR)                         │
│ - Détails, durée, traçabilité                                │
│ - Aucun nettoyage persistance (MiyuClock ne persiste pas)      │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Suites de tests

### 7.1 Suites prédéfinies

| Suite | Tests inclus | Durée estimée | Usage |
|-------|--------------|---------------|-------|
| **Quick** | MCLOCK-N-001, MCLOCK-N-002, MCLOCK-D-001, MCLOCK-D-002 | < 1 min | Vérification rapide |
| **Standard** | Tous MCLOCK-N-*, MCLOCK-D-* | 1–3 min | Vérification quotidienne |
| **Full** | Tous MCLOCK-* (now + delta, tous cas) | 2–5 min | Vérification complète |

---

## 8. Références croisées

| Document | Lien |
|----------|------|
| MiyuClock - Documentation Fondatrice | [MiyuClock - Documentation Fondatrice](../../MiyuClock%20-%20Documentation%20Fondatrice.md) |
| MiyuClock - Reference Outils | [MiyuClock - Reference Outils](../../MiyuClock%20-%20Reference%20Outils.md) |
| MiyuClock - Cycle Tests Contract | [MiyuClock - Cycle Tests Contract](./MiyuClock%20-%20Cycle%20Tests%20Contract.md) |
| MiyuClock - KindMother Integration Contract | [MiyuClock - KindMother Integration Contract](../integration/MiyuClock%20-%20KindMother%20Integration%20Contract.md) |
| Lois Autonomie (LOI-4) | [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de référence
