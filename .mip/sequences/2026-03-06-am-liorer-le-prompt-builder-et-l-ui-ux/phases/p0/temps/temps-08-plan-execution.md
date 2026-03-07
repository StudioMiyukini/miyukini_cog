# P0 Temps 8 - Plan execution

## Statut

- Etat : TERMINE
- Phase : P0 Temps 8
- Responsable principal : Denis

## TL;DR

Plan complet dans plans_p3/...plan.md. C4 confirme. 5 etapes (E00-E04) + BUF, 15 taches. E01 (Rust) et E02 (HTML/CSS) en parallele apres E00. Agents : Denis + Francois + Lise.

## DAG des etapes

```
E00 --> E01 + E02 (parallele) --> E03 --> E04 --> BUF
```

## Complexite sequence

**C4** — feature majeure enrichie : 5 fichiers, 15 taches, 2 agents dev en parallele, layout UI complet a refaire, backend Rust etendu.

## Estimation par etape

| Etape | Taches | Agents | Notes |
|-------|--------|--------|-------|
| E00 | 1 | Denis | Smoke test RED |
| E01 | 4 | Francois | models.rs + api.rs + tests |
| E02 | 3 | Lise | HTML/CSS bi-panneaux |
| E03 | 4 | Lise | Options avancees + preview live |
| E04 | 3 | Lise + Denis | localStorage + polish + checkpoint |
| BUF | 3 | Francois + Lise | Corrections post-P4/P5 |

## Risques execution

- Template JS/Rust desynchronise : documenter et relire systematiquement
- CSS layout bi-panneaux : tester sur differentes resolutions avant P4
- Cargo clippy strict : appliquer clippy apres chaque etape, pas seulement en P4
