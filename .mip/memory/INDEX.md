<!-- @id mem.index.routing
     @do route_memory_file_lookup_by_task
     @role index
     @layer memory
     @human Index memoire - routage chargement selectif -->

# Memoire MIP - Index

> Chargement selectif : charger uniquement les fichiers pertinents a la tache.

## Flux de chargement

```
Tache (ex: refactor MGE) -> INDEX.md -> project-mge.md, patterns-and-lessons.md
Tache (ex: P0 cadrage)  -> MEMORY.md (vue d'ensemble)
Tache (ex: P3 implementation) -> patterns-and-lessons.md (obligatoire)
```

## Routage par fichier

| Fichier | Contenu | Quand charger | @id |
|---------|---------|---------------|-----|
| MEMORY.md | Vue d'ensemble, patterns top 5, erreurs top 3, etats | P0, P3, P6 | mem.master.cog |
| patterns-and-lessons.md | Patterns, erreurs, anti-patterns | Avant P3, review code | mem.patterns.lessons |
| project-mge.md | MGE decisions, architecture 4 couches, render reforge | Taches MGE | mem.project.mge |
| project-miyuki-ui.md | Miyuki UI (tokens, dioxus, egui) | Taches UI, migration | mem.project.miyuki_ui |
| project-miyucloud.md | MiyuCloud architecture, crypto, defauts en attente | Taches MiyuCloud | mem.project.miyucloud |
| certifications-index.md | Index certifications | Audit, P0, P6 | mem.index.certifications |
| user-profile.md | Mode autonomie, preferences utilisateur | P0, changement mode | mem.profile.user |
| project-file-map.md | Cartographie des fichiers du repo | P0 T4, P3, P4 | mem.map.project_file_map |
| stack-patterns.md | Patterns techniques valides par stack | P0 T6, P3 | mem.patterns.stack |
| stack-cheatsheet.md | Cheatsheet framework principal | P3 | mem.ref.stack_cheatsheet |
| api-contracts.md | Contrats API (schemas, erreurs, versions) | P0 T6, P3, P4 | mem.contracts.api |
| test-templates.md | Templates de tests unit/integration/e2e | P3 | mem.templates.tests |
| code-annotations-templates.md | Templates d'annotations code | P3, P4 | mem.templates.annotations |
| security-patterns.md | Patterns securite approuves | P0 T5, P3, P4 | mem.security.patterns |
| mip-decisions.md | Decisions MIP/projet datees | P0 T4/T8, P6 | mem.decisions.log |
| mip-performance-history.md | Historique efficience (tokens, duree, actions) | P4, P6 | mem.metrics.history |
| team-skills-audit.md | Matrice competences agents et ecarts | P0 T8, P6 | mem.team.skills |
| mip-lessons.md | Lecons apprises par sequence | P6 | mem.lessons.sequences |
| agent-tuning.md | Reglages prompts/modeles/outils par agent | P4, P6 | mem.tuning.agents |

## Regles

- Les fichiers de baseline memoire ci-dessus sont obligatoires. Si absent, SETUP doit les creer en mode template.
- MEMORY.md : point d'entree, <=200 lignes.
- project-* : un fichier par chantier majeur.
- patterns-and-lessons.md : toujours charger avant P3.
- user-profile.md : charger pour adapter le niveau d'autonomie.

## Reference

- Normes ecriture : `.mip/memory/SCHEMA.md`
