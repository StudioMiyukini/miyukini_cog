# P0 Temps 8 - Plan execution

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Denis

## DAG P3

```
E00 (infra + composants) → E01 (JayFestival) → E03 (JayKonta) → BUF
                        → E02 (JayXpose)    → E04 (JayManga)
                                            → E05 (JayKoa)
```

E01 // E02 // (E03 apres E00 uniquement — pas de dependance entre services)

## Etapes

| Etape | Titre | Agent | Taches | Depend |
|-------|-------|-------|--------|--------|
| E00 | Infrastructure + nouveaux composants miyuki-ui-dioxus | Francois + Lise | 8 | — |
| E01 | JayFestival UI refonte (15 fichiers) | Lise | 15 | E00 |
| E02 | JayXpose UI refonte (11 fichiers) | Lise | 11 | E00 |
| E03 | JayKonta UI refonte (8 fichiers) | Hugo | 8 | E00 |
| E04 | JayManga UI refonte (14 fichiers) | Hugo | 14 | E00 |
| E05 | JayKoa UI refonte (7 fichiers) | Denis | 7 | E00 |
| BUF | Corrections clippy + inline styles residuels | Victor + Denis | variable | E01-E05 |

Total taches estimees : ~65 + BUF

## Criteres de completion globaux

- 0 inline `style:` hardcode dans les 55 fichiers cibles
- 0 couleur hex hardcodee hors variables CSS
- `cargo clippy -p miyukini-central -- -D warnings` : 0 violations
- `cargo check -p miyukini-central` : 0 erreurs
- MSCM conforme sur 100% des fichiers modifies
