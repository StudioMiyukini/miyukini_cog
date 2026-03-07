# E01 — Backend : derive_status + progress etendu + artefacts badges

## Statut : Terminé
## Depend de : E00
## Agents : Francois
## Taches : 4

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E01-01 | FEAT | Ajouter `derive_status(seq_dir)` — lit p6-trace.md, retourne done/archived/active | Francois | src/api.rs | pending |
| E01-02 | FEAT | Appliquer derive_status dans sequences_handler (fallback si status vide) | Francois | src/api.rs | pending |
| E01-03 | FEAT | Etendre progress_handler : P4 (audits/pass-*.md), P5 (p5-trace.md TERMINE), P6 (p6-trace.md) | Francois | src/api.rs | pending |
| E01-04 | FEAT | Ajouter champ `done: bool` dans artefacts_handler (marker "Etat : TERMINE") | Francois | src/api.rs, src/models.rs | pending |

## Notes
- derive_status : chercher "SUCCES" → "done", "REFUSE"/"abandonne" → "archived", sinon "active"
- progress : nouvelles phases P4/P5/P6 retournees dans le JSON
- artefacts : `files` devient Vec<{path, done}> au lieu de Vec<String>

## Commit message template
`feat(mipower): E01 -- derive_status + progress P4/P5/P6 + artefacts badges`
