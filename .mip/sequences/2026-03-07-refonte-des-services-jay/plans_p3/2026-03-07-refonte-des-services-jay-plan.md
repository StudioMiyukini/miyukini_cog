# Plan P3 2026-03-07-refonte-des-services-jay

## Statut

- Etat : A faire (P0 terminé)
- Phase : P3
- Complexite : **C5** — stratégique
- Responsable principal : Denis

## TL;DR

Plan C5 — 5 étapes + BUF. Vague 1 MASS : E00 (smoke test) + E01 (MSCM audit). Vague 2 MASS : E02 (JayFestival) + E03 (JayXpose). Vague 3 : E04 (COG Web Portal). BUF : corrections. ~40-60 tâches atomiques au total. Mode FULL autopilot.

## DAG des étapes

```
E00 (smoke test, séquentiel)
  └─> E01 (MSCM audit, MASS Georges+François+Lise)
       └─> E02 (JayFestival, MASS Lise+François) ─┐
           E03 (JayXpose+contrats, MASS)          ─┤ [vague 2 parallèle]
                                                   └─> E04 (COG Web Portal)
                                                        └─> BUF
```

## Étapes

| Étape | Nom | Agents | Tâches est. | Dépend de | Fichier |
|-------|-----|--------|-------------|-----------|---------|
| E00 | Smoke test + Init workspace | Denis | 3 | — | `etapes/etape-00.md` |
| E01 | MSCM Audit famille Jay | George+François+Lise | 10-12 | E00 | `etapes/etape-01.md` |
| E02 | JayFestival prod-ready | Lise+François+Victor | 12-15 | E01 | `etapes/etape-02.md` |
| E03 | JayXpose prod-ready + Contrats | Lise+François | 10-12 | E01 | `etapes/etape-03.md` |
| E04 | COG Web Portal | François+Victor+Lise | 10-12 | E03 | `etapes/etape-04.md` |
| BUF | Buffer corrections + Audit P4 | Denis+George+Victor | 5-8 | E04 | `etapes/etape-buf.md` |

## Agents mobilisés

| Agent | Rôle | Étapes |
|-------|------|--------|
| Denis | Coordination MASS, checkpoints /5 | E00, BUF |
| George | Audit MSCM, conformité | E01, BUF |
| François | Back Rust, Portal, contrats | E01, E02, E03, E04 |
| Lise | UI Dioxus 0.7, design system | E01, E02, E03, E04 |
| Victor | Sécurité Jay + Portal | E02, E04, BUF |
| Jean | Efficience tokens, checkpoint | BUF |

## Règles MASS

- Vague 1 : E00 séquentiel (prérequis workspace)
- Vague 2 : E01 parallèle (Georges+François+Lise sur modules différents)
- Vague 3 : E02+E03 parallèle (JayFestival et JayXpose = crates disjoints)
- Vague 4 : E04 (Portal — dépend E03 pour contrats)
- Checkpoint Denis toutes les 5 tâches : mini-audit + push

## Risques P3

[A completer]

## Criteres de sortie P3

- [ ] Toutes les etapes Terminees
- [ ] `cargo test` : 0 failed
- [ ] `cargo clippy -D warnings` : 0 violations
- [ ] Score securite >= 90/100
- [ ] Audit efficience >= 15/20

