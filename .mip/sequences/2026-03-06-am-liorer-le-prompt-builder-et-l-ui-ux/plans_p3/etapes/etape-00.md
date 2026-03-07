# E00 -- Test fumee (Smoke Test)

## Statut : A faire
## Depend de : --
## Agents : Denis
## Taches : 1
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E00-01 | TEST | Smoke test : ajouter champs manquants dans PromptBuilderInput (compile mais test echoue sur template incomplet) | Denis | src/models.rs, src/api.rs | pending | | |

## Notes
Test fumee : creer un `PromptBuilderInput` avec les champs `autonomy_mode`, `urgency`, `sensitive_data`, `msw_toggle` dans un test RED. Ce test DOIT echouer avant E01 (champs absents de la struct actuelle). Valide la structure du plan TDD.

## Commit message template
`test(mipower): E00 -- smoke test prompt builder v2 (RED)`
