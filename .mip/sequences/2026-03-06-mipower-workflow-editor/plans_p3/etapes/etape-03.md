# E03 -- Dashboard frontend

## Statut : Terminé
## Depend de : E01 (parallele avec E02)
## Agents : Lise
## Taches : 5
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E03-01 | CODE | Creer Sidebar.svelte : navigation (Dashboard / Sequences / Parametres) + dark mode toggle | Lise | src/lib/components/Sidebar.svelte | pending | -- | -- |
| E03-02 | CODE | Creer store sequences.ts : appel IPC list_sequences -> $state rune | Lise | src/lib/stores/sequences.ts | pending | -- | -- |
| E03-03 | CODE | Creer SequenceCard.svelte : affiche slug, date, statut badge (actif=bleu/termine=vert/archive=gris), T/C class | Lise | src/lib/components/SequenceCard.svelte | pending | -- | -- |
| E03-04 | CODE | Creer Dashboard.svelte : grille de SequenceCard, input filtre/recherche, tri par date | Lise | src/lib/components/Dashboard.svelte | pending | -- | -- |
| E03-05 | TEST-U | Tests Svelte (vitest) : SequenceCard rendu correct, filtre fonctionne, badges corrects | Lise | src/lib/components/SequenceCard.test.ts | pending | -- | -- |

## Commit message template
`feat(mipower): E03 -- dashboard sequences frontend`

## Criteres de completion
- Dashboard affiche les sequences du store
- Filtre/recherche par slug fonctionne
- Badges statut colores correctement
- Dark mode actif par defaut
- Tests Svelte passent
