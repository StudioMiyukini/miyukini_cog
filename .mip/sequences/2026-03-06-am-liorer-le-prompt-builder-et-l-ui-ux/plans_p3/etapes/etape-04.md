# E04 -- localStorage + polish CSS + integration finale

## Statut : A faire
## Depend de : E03
## Agents : Lise + Denis (checkpoint)
## Taches : 3
## Commencé : [dd/mm/yyyy - hh:mm]
## Fini : [dd/mm/yyyy - hh:mm]

| # | Cat | Titre | Agent | Fichier(s) | Statut | Commencé | Fini |
|---|-----|-------|-------|------------|--------|----------|------|
| E04-01 | CODE | localStorage : sauvegarder tous les champs au change + restaurer au DOMContentLoaded | Lise | static/app.js | pending | | |
| E04-02 | CODE | Polish CSS : chips tags, grille agents, toggles, details summary, responsive final | Lise | static/app.css | pending | | |
| E04-03 | TEST | Checkpoint Denis : integration complète, test manuel 10 criteres acceptance, cargo test | Denis | all | pending | | |

## Notes
Cles localStorage : pb_title, pb_class, pb_domain, pb_complexity, pb_autonomy, pb_stack, pb_constraints, pb_agents (JSON), pb_tags (JSON), pb_urgency, pb_sensitive, pb_msw.
Integration finale : connecter le formulaire enrichi avec le bouton Generer existant (POST /api/prompt avec tous les nouveaux champs).

## Criteres de completion
- Config restauree au reload (localStorage)
- 10 criteres d'acceptance spec tous valides
- cargo test -p mipower : 0 failed

## Commit message template
`feat(mipower): E04 -- localStorage + polish CSS + integration finale`
