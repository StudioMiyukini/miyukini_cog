# P0 Temps 7 - Generation agents fine-tuned

## Statut

- Etat : TERMINE
- Phase : P0 Temps 7
- Responsable principal : Maria

## TL;DR

4 agents fine-tuned generes. P3 : Francois (back Rust, models+api) + Lise (front HTML/CSS/JS, UI complete). P4 : Victor (securite, score >=88/100) + George (conformite, clippy+tests+UX).

## Agents generes

| Agent | Phase | Specialite | Fichier |
|-------|-------|-----------|---------|
| Francois | P3 | models.rs + api.rs : PromptBuilderInput etendu, validations, template prompt | agents/P3_francois.md |
| Lise | P3 | index.html + app.js + app.css : bi-panneaux, champs, preview live, agents grid, tags, localStorage | agents/P3_lise.md |
| Victor | P4 | Audit securite /100 (path traversal, Content-Type, validations, cargo audit) | agents/P4_victor.md |
| George | P4 | Audit conformite (clippy, tests, lint, annotations, Lois Autonomie, UX) | agents/P4_george.md |

## Base d'apprentissage

- T04 inventaire (5 fichiers cibles, 0 nouvelle dep)
- T05 securite (validations longueur, whitelist agents, cargo audit)
- T06 specification (schema PromptBuilderInput, template canonique, criteres acceptance)
