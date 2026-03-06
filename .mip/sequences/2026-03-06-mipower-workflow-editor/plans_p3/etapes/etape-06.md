# E06 -- Prompt builder

## Statut : Terminé
## Depend de : E04
## Agents : Lise (UI) + Francois (backend generate_prompt + init_sequence)
## Taches : 5
## Commencé : 07/03/2026 - 00:00
## Fini : 07/03/2026 - 00:00

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E06-01 | CODE | Backend : `POST /api/prompt` genere le prompt MIP depuis PromptBuilderInput | Francois | apps/mipower/src/api.rs | done | 07/03/2026 | 07/03/2026 |
| E06-02 | CODE | Backend : `POST /api/init-sequence` appelle init-sequence-by-complexity.ps1 via std::process::Command avec validation slug+complexite | Francois | apps/mipower/src/api.rs | done | 07/03/2026 | 07/03/2026 |
| E06-03 | CODE | Frontend : formulaire prompt builder (titre, classe, domaine, description, contraintes, stack) | Lise | apps/mipower/static/index.html | done | 07/03/2026 | 07/03/2026 |
| E06-04 | CODE | Frontend : textarea copiable + bouton Init sequence (complexite selector) | Lise | apps/mipower/static/app.js | done | 07/03/2026 | 07/03/2026 |
| E06-05 | TEST-U | Tests : test_generate_prompt_non_empty + test_init_sequence_slug_validation | Francois | apps/mipower/src/api.rs | done | 07/03/2026 | 07/03/2026 |

## Commit message template
`feat(mipower): E06 -- prompt builder + init sequence`

## Criteres de completion
- Remplir le formulaire -> prompt MIP complet genere et affiche
- Bouton "Copier" place le prompt dans le clipboard
- Bouton "Init sequence" cree l'arborescence .mip/ (si mip_root configure)
- Slug invalide -> message d'erreur clair dans l'UI
- Tests passent
