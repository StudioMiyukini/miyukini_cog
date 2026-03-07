# P0 Temps 7 - Generation agents fine-tuned

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Maria

## Agents P3

| Agent | Role | Etapes |
|-------|------|--------|
| Lise | UI Dioxus — refonte JayFestival + JayXpose | E00 (composants), E01, E02 |
| Hugo | UI Dioxus — refonte JayKonta + JayManga | E03, E04 |
| Denis | UI Dioxus — refonte JayKoa + coordination | E05, BUF |
| Francois | Infrastructure provide_context, composants miyuki-ui-dioxus, clippy | E00, support |
| Victor | Smoke tests, clippy -D warnings, MSCM audit | E00-E05 verification |
| George | Audit global P4 | P4 |
| Jean | Audit efficience P4 | P4 |
| Arianne | Faisabilite, P6 | P0-T09, P6 |

## Instructions agents

### Lise (UI)
- Lire chaque fichier avant d'editer
- Pattern systematique : `let p = use_context::<miyuki_ui_tokens::Palette>()`
- Remplacer inline styles par CSS vars ou composants miyuki-ui-dioxus
- Garder toute la logique Dioxus intacte — migrer le style uniquement
- MSCM obligatoire sur chaque fichier modifie

### Francois (infra)
- E00 en premier : provide_context dans App() + nouveaux composants
- Verifier cargo check apres chaque etape
- clippy -D warnings apres chaque etape
