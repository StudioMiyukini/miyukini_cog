# P0 Temps 8 - Plan execution

## Statut

- Etat : Terminé
- Phase : P0 Temps 8
- Agent : Denis
- Date : 2026-03-07

## TL;DR

Plan C5, 5 étapes + BUF, ~59 tâches atomiques, MASS 4 vagues. Plan détaillé dans `plans_p3/etapes/`. Mode FULL autopilot.

## DAG des étapes

```
E00 (3 tâches) — smoke test + init workspace
  └─> E01 (12, MASS) — MSCM audit famille Jay
       ├─> E02 (14, MASS Lise+François) ─┐ [parallèle]
       └─> E03 (12, MASS Lise+François) ─┘
                                          └─> E04 (12) — COG Web Portal
                                               └─> BUF (6+) — audit + corrections
```

## Complexité séquence

**C5 — stratégique** (confirmé Denis T8)

Justification : nouveau service web (COG Web Portal) + refonte prod-ready 2 services + audit MSCM complet 8 services + architecture générique extensible + sécurité DURCI.

## Estimation par étape

| Étape | Tâches | Agents | Complexité |
|-------|--------|--------|-----------|
| E00 | 3 | Denis | C2 |
| E01 | 12 | George+François+Lise | C4 |
| E02 | 14 | Lise+François+Victor | C4 |
| E03 | 12 | Lise+François | C4 |
| E04 | 12 | François+Victor+Lise | C4 |
| BUF | 6+ | Denis+George+Victor+Jean | C3 |
| **Total** | **~59** | 6 agents | **C5** |

## Risques exécution

| Risque | Mitigation |
|--------|------------|
| Scope glissement | Gate MVP : E00+E01+E02+E04 = MVP. E03 = V1. |
| MSCM audit 8 services trop long | E01 = corrections JayFestival+JayXpose only en P3. |
| Dioxus 0.7 patterns inattendus | Spot-check Context7 avant chaque tâche Lise. |
| Frein d'urgence | Arrêt auto si bug bloquant après 2 tentatives. |

