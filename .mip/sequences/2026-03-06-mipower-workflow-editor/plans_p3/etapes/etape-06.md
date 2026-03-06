# E06 -- Prompt builder

## Statut : A faire
## Depend de : E04
## Agents : Lise (UI) + Francois (backend generate_prompt + init_sequence)
## Taches : 5
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E06-01 | CODE | Backend : commande `generate_prompt(input: PromptBuilderInput) -> String` : template MIP premier prompt depuis les champs du formulaire | Francois | src-tauri/src/commands/prompt_builder.rs | pending | -- | -- |
| E06-02 | CODE | Backend : commande `init_sequence(slug, mip_root) -> Result<String>` : appelle init-sequence-base.ps1 via std::process::Command avec args valides uniquement | Francois | src-tauri/src/commands/prompt_builder.rs | pending | -- | -- |
| E06-03 | CODE | Frontend : creer PromptBuilder.svelte : formulaire (titre, classe T3/T4/T5, domaine, description, contraintes, stack, tags) | Lise | src/lib/components/PromptBuilder.svelte | pending | -- | -- |
| E06-04 | CODE | Frontend : afficher le prompt genere dans une textarea copiable (bouton copier clipboard) | Lise | src/lib/components/PromptBuilder.svelte | pending | -- | -- |
| E06-05 | TEST-U | Tests : generate_prompt avec input minimal -> prompt non vide; init_sequence avec slug invalide -> erreur retournee (pas de crash) | Francois | src-tauri/tests/prompt_builder_tests.rs | pending | -- | -- |

## Commit message template
`feat(mipower): E06 -- prompt builder + init sequence`

## Criteres de completion
- Remplir le formulaire -> prompt MIP complet genere et affiche
- Bouton "Copier" place le prompt dans le clipboard
- Bouton "Init sequence" cree l'arborescence .mip/ (si mip_root configure)
- Slug invalide -> message d'erreur clair dans l'UI
- Tests passent
