# Modules MIP — Index

## TL;DR

Source unique des modules MIP. Tous les chemins sont relatifs à la racine du projet.

---

## Modules disponibles

| Module | Chemin | Charger quand |
|--------|--------|---------------|
| setup | `.mip/modules/setup.md` | `.mip/environment.md` absent ou `/mip_setup` |
| p0-details-index | `.mip/modules/p0-details-index.md` | Début P0 (T3+) — toujours en premier |
| p0-details | `.mip/modules/p0-details.md` | Drill-down par temps (Read offset/limit) |
| p3-execution | `.mip/modules/p3-execution.md` | Début P3 |
| p4-p5-p6 | `.mip/modules/p4-p5-p6.md` | Début P4 |
| metrics | `.mip/modules/metrics.md` | Init métriques |
| mass | `.mip/modules/mass.md` | T4-T5 avec parallélisation |
| **agent-context-map** | `.mip/modules/agent-context-map.md` | **Chargement minimal par phase/agent — lire avant toute tâche** |
| tools-reference | `.mip/modules/tools-reference.md` | Sur demande |

---

## Référence

- Skill workflow : `.mip/skills/miyukini-mip-workflow/SKILL.md`
- Conventions : `.mip/protocol/conventions.md`
