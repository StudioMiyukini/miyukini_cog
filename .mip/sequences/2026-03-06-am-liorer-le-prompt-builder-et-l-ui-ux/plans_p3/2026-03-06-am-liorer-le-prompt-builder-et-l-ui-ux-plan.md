# Plan P3 — am-liorer-le-prompt-builder-et-l-ui-ux

## Statut

- Etat : A faire
- Phase : P3
- Complexite : C4 — elevee
- Responsable principal : Denis

## TL;DR

15 taches, 5 etapes (E00-E04) + BUF. E01 (Rust) et E02 (HTML/CSS) en parallele. Agents : Francois (back) + Lise (front). Checkpoint Denis en E04. Score securite cible >= 88/100.

## DAG des etapes

```
E00 (smoke) --> E01 (backend Rust) ---+
            \                          +--> E03 (options avancees + preview) --> E04 (localStorage + polish) --> BUF
             --> E02 (UI base HTML) --+
```

E01 et E02 executables en parallele (fichiers disjoints).

## Etapes

Voir `etapes/index.md` pour le detail complet.

| Etape | Titre | Taches | Agent(s) | Depend de |
|-------|-------|--------|----------|-----------|
| E00 | Test fumee (smoke test RED) | 1 | Denis | -- |
| E01 | Backend enrichi (models.rs + api.rs) | 4 | Francois | E00 |
| E02 | UI base : bi-panneaux + champs enrichis | 3 | Lise | E00 |
| E03 | Options avancees + preview live | 4 | Lise | E01, E02 |
| E04 | localStorage + polish + integration finale | 3 | Lise + Denis | E03 |
| BUF | Buffer corrections (20%) | 3 | Francois + Lise | E04 |

## Agents mobilises

| Agent | Role | Etapes |
|-------|------|--------|
| Denis | Chef Dev — smoke test, checkpoint E04, coordination | E00, E04 |
| Francois | Dev Back-End — PromptBuilderInput, api.rs, tests | E01, BUF |
| Lise | Dev Front-End — index.html, app.js, app.css | E02, E03, E04, BUF |
| Victor | Audit Securite — P4 | P4 |
| George | Audit Conformite — P4 | P4 |

## Risques P3

| Risque | Impact | Mitigation |
|--------|--------|-----------|
| Template JS desynchronise du template Rust | MED | Template documente en spec comme source de verite unique |
| CSS bi-panneaux casse sur petits ecrans | LOW | Responsive flex-column < 900px |
| Debounce trop agressif | LOW | 300ms suffit, pas d'appel API en preview |

## Criteres de sortie P3

- [ ] Toutes les etapes Terminees
- [ ] `cargo test -p mipower` : 0 failed (15 tests dont 2 nouveaux)
- [ ] `cargo clippy -p mipower -- -D warnings` : 0 violation
- [ ] Score securite P4 >= 88/100
- [ ] 10 criteres d'acceptance spec valides
