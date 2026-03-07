# P0 Temps 3 - Analyse concurrentielle

## Statut

- Etat : TERMINE
- Phase : P0 Temps 3
- Responsable principal : Fabrice

## TL;DR

Pas de concurrent direct (MIPOWER est un outil MIP-specifique). Les references UX sont : GitHub Copilot prompt panels, Linear issue forms, Raycast AI commands. La solution retenue (preview live + sections retractables) est alignee avec les meilleurs patterns du marche.

## Solutions existantes

| Solution | Points forts | Points faibles | Pertinence |
|----------|-------------|---------------|-----------|
| GitHub Copilot Chat | Preview inline, contexte code, multi-options | Lie a VS Code, pas de formulaire structure | UX reference pour le preview live |
| Linear (creation issue) | Formulaire riche, tags multiselect, assignees, priorite, shortcut keyboard | SaaS, React lourd | UX reference pour champs riches + tags |
| Raycast AI Commands | Params types, preview instantane, select predifinis | Macos uniquement, closed source | UX reference pour params + preview |
| OpenWebUI (system prompt builder) | Multiselect, sections avancees repliables, preview | Lourd, Node.js | Reference pattern accordion + preview |
| Cursor Composer / Windsurf | Zone prompt enrichie avec @-mentions | Pas de formulaire structure | Reference pour mode autonomie selector |

## Positionnement

MIPOWER Prompt Builder v2 est unique : formulaire structure MIP-specifique + preview live + init sequence integre. Aucun outil du marche ne cible le protocole MIP. La valeur est dans la rapidite de saisie et la completude du prompt genere (conformite MIP garantie). Le pattern UI le plus adapte est Linear-style (champs riches) + Raycast-style (preview instantane).

