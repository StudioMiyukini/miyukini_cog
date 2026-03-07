# E01 -- Backend enrichi (Rust)

## Statut : A faire
## Depend de : E00
## Agents : Francois
## Taches : 4
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E01-01 | CODE | Etendre PromptBuilderInput (autonomy_mode, urgency, sensitive_data, msw_toggle) | Francois | src/models.rs | pending | | |
| E01-02 | CODE | Ajouter validations (longueur, whitelist task_class/domain/agents/autonomy_mode) dans prompt_handler | Francois | src/api.rs | pending | | |
| E01-03 | CODE | Enrichir template prompt (lignes optionnelles agents, tags, mode, urgence, etc.) | Francois | src/api.rs | pending | | |
| E01-04 | TEST | Mettre a jour tests existants + ajouter test_generate_prompt_with_agents + test_generate_prompt_with_autonomy_mode | Francois | src/api.rs | pending | | |

## Criteres de completion
- `cargo test -p mipower` : 0 failed, 2 nouveaux tests passes
- `cargo clippy -p mipower -- -D warnings` : 0 warning
- Nouveau champ dans PromptBuilderInput serde Deserialize OK

## Commit message template
`feat(mipower): E01 -- backend PromptBuilderInput v2 + validations + template enrichi`
