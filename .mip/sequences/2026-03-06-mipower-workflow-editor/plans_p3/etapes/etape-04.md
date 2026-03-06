# E04 -- Lecteur rapport (Must work #1)

## Statut : A faire
## Depend de : E03
## Agents : Lise
## Taches : 5
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E04-01 | CODE | Creer utils/markdown.ts : marked.js + DOMPurify.sanitize() + mermaid.run() sur code blocks ```mermaid``` | Lise | src/lib/utils/markdown.ts | pending | -- | -- |
| E04-02 | CODE | Creer ReportViewer.svelte : recoit path artefact -> IPC read_artefact -> renderMarkdown -> {@html sanitized} | Lise | src/lib/components/ReportViewer.svelte | pending | -- | -- |
| E04-03 | CODE | Sidebar artefacts : arbre des fichiers (briefs/ specs/ plans_p3/ audits/ rapports_finaux/) de la sequence active, click = ouvre dans ReportViewer | Lise | src/lib/components/ArtifactTree.svelte | pending | -- | -- |
| E04-04 | CODE | Sommaire automatique : extraire les H2 du MD -> liste de liens ancre dans un panel droit (navigation intra-rapport) | Lise | src/lib/components/ReportViewer.svelte | pending | -- | -- |
| E04-05 | TEST-U | Test : un .md avec tableau Markdown et bloc mermaid est rendu correctement (snapshot test) ; XSS <script> dans MD est neutralise par DOMPurify | Lise | src/lib/utils/markdown.test.ts | pending | -- | -- |

## Commit message template
`feat(mipower): E04 -- lecteur rapport MD riche (Mermaid + tableaux)`

## Criteres de completion
- Ouvrir n'importe quel .md MIP -> rendu lisible avec tableaux + diagrams Mermaid
- Navigation via sommaire H2 fonctionnelle
- DOMPurify bloque <script>alert()</script> injecte dans le MD
- Arbre artefacts de la sequence visible et navigable
- Tests XSS + rendu passent
