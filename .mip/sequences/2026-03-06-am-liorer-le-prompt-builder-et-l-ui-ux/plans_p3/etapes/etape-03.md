# E03 -- Options avancees + Preview live

## Statut : A faire
## Depend de : E01, E02
## Agents : Lise
## Taches : 4
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E03-01 | CODE | Section Options avancees : details HTML natif + grille agents 2 colonnes (10 checkboxes MIP) | Lise | static/index.html, static/app.css | pending | | |
| E03-02 | CODE | Tags chips : input pb-tag-input + bouton Ajouter + liste chips cliquables (supprimer), max 10 | Lise | static/index.html, static/app.js, static/app.css | pending | | |
| E03-03 | CODE | Toggles HTML : urgence, donnees sensibles, MSW (Mode Sans Web) | Lise | static/index.html | pending | | |
| E03-04 | CODE | Preview live : implementer buildPromptLocal() (template JS miroir spec) + debounce 300ms sur tous les champs | Lise | static/app.js | pending | | |

## Notes
buildPromptLocal() doit reproduire exactement le template Rust de prompt_handler. Lignes optionnelles selon spec. Preview via textarea.value (pas innerHTML). Debounce 300ms.

## Criteres de completion
- Section avancee repliee par defaut, s'ouvre au clic
- Agents selectionnes → apparaissent dans preview live
- Tags ajoutes/supprimes → preview mise a jour
- Preview mise a jour <= 400ms apres toute modification

## Commit message template
`feat(mipower): E03 -- options avancees (agents, tags, toggles) + preview live`
