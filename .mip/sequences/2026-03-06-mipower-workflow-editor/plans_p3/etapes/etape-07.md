# E07 -- Polish UI + design system

## Statut : A faire
## Depend de : E06
## Agents : Lise
## Taches : 4
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E07-01 | CODE | Design system : palette Miyukini (bleu #3B82F6, violet #8B5CF6, fond #0F172A dark, #F8FAFC light), tokens Tailwind config | Lise | tailwind.config.ts, src/app.css | pending | -- | -- |
| E07-02 | CODE | Composants : hover states, transitions (150ms ease), focus-visible rings, scrollbar custom dark | Lise | src/lib/components/*.svelte | pending | -- | -- |
| E07-03 | CODE | Responsive sidebar : repliable sur click (icone seulement en mode compact), persiste en localStorage | Lise | src/lib/components/Sidebar.svelte | pending | -- | -- |
| E07-04 | TEST-U | Tests accessibilite : tous les elements interactifs ont aria-label ou aria-labelledby (audit axe-core ou vitest-a11y) | Lise | src/lib/components/*.test.ts | pending | -- | -- |

## Commit message template
`feat(mipower): E07 -- polish UI design system Miyukini`

## Criteres de completion
- Palette Miyukini appliquee coheremment
- Sidebar repliable et persistante
- Transitions fluides (pas de flash layout)
- 0 erreur axe-core major
