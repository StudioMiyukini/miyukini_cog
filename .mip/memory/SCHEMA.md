# Memoire MIP - Schema et normes d'ecriture

<!-- @id mem.schema
     @do define_memory_structure_and_writing_norms
     @role config
     @layer config
     @human Schema memoire MIP - structure, index, MSCM -->

> Ce document definit les normes d'ecriture des fichiers memoire : structure, annotations MSCM, index, nomenclature.

---

## Structure

```
.mip/memory/
├── SCHEMA.md
├── INDEX.md
├── MEMORY.md
├── user-profile.md
├── certifications-index.md
├── patterns-and-lessons.md
├── project-mge.md
├── project-miyuki-ui.md
├── project-miyucloud.md
├── project-file-map.md
├── stack-patterns.md
├── stack-cheatsheet.md
├── api-contracts.md
├── test-templates.md
├── code-annotations-templates.md
├── security-patterns.md
├── mip-decisions.md
├── mip-performance-history.md
├── team-skills-audit.md
├── mip-lessons.md
├── agent-tuning.md
└── {project-<slug>.md}
```

---

## Annotations MSCM obligatoires

Chaque fichier memoire doit commencer par un bloc MSCM :

```markdown
<!-- @id mem.<type>.<slug>
     @do <action_snake_case>
     @role <role>
     @layer memory
     @human <Description courte pour humains> -->
```

### Convention @id

| Prefixe | Type | Exemples |
|---------|------|----------|
| `mem.master` | Vue d'ensemble | `mem.master.cog` |
| `mem.profile` | Profil utilisateur | `mem.profile.user` |
| `mem.patterns` | Patterns et lecons | `mem.patterns.lessons` |
| `mem.project` | Projet/chantier | `mem.project.mge`, `mem.project.miyucloud` |
| `mem.index` | Index routage | `mem.index.certifications` |

### @role

| role | Usage |
|------|-------|
| `overview` | Vue d'ensemble (MEMORY.md) |
| `profile` | Profil utilisateur |
| `patterns` | Patterns, erreurs, anti-patterns |
| `project` | Decisions projet, architecture |
| `index` | Index, routage |
| `config` | Configuration memoire |

### @layer

Toujours `memory` pour les fichiers sous `.mip/memory/`.

---

## Baseline memoire minimale obligatoire

Les fichiers suivants doivent exister apres SETUP :

`project-file-map.md`, `stack-patterns.md`, `stack-cheatsheet.md`, `api-contracts.md`, `test-templates.md`, `code-annotations-templates.md`, `security-patterns.md`, `mip-decisions.md`, `mip-performance-history.md`, `team-skills-audit.md`, `mip-lessons.md`, `agent-tuning.md`.

Si une donnee n'est pas encore connue, le fichier est cree avec sections `A completer` plutot que laisse absent.

---

## Regles d'ecriture

1. Francais (hors noms techniques/certifications).
2. TL;DR : 1-2 lignes en bloc `>` pour tout fichier >=30 lignes.
3. Date de confirmation : format `(confirme mars 2026)`.
4. Pas de duplication : un fait = un fichier.
5. Anti-patterns numerotes `AP-NN`.
6. Chemins relatifs `.mip/memory/`.

---

## Reference MSCM

- Schema protocole : `.mip/skills/miyukini-mip-workflow/mscm/schema.md`
- Convention @id : `mem.<type>.<slug>`
