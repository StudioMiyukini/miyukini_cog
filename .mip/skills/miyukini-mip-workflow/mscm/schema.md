# Protocole MIP — Schéma MSCM

<!-- @id mip.mscm.schema
     @do define_mscm_annotations_for_protocol
     @role config
     @layer config
     @human Schéma MSCM du protocol MIP -->

> Référence @id, @role, @layer. Plan : `<sequence>/plans_p3/`.

## Convention @id

```
mip.<phase>.<unite>.<bloc>
```

| Préfixe | Exemples |
|---------|----------|
| mip.core | invariants, classification, nomenclature |
| mip.setup | s1.scan, s4.outil |
| mip.p0 | rules, t1.brainstorm, t6.spec, questionnaire |
| mip.p3 | tdd.cycle, smoke, checkpoint, back, front |
| mip.p4, p5, p6 | denis, george, victor, gate |
| mip.mass | dag.format, dispatch, merge, loi9 |
| mip.metrics | init, collecte, tokens |
| mip.agent | maria, denis, francois, lise... |
| mip.gate | p0, p3_p4, p4_p5, p5 |

## @role

| role | Rôle |
|------|-------|
| gate | Critère passage phase |
| rule | Règle non-négociable |
| orchestration | Flux, sync |
| metric | Métriques |
| agent | Spécifique agent |
| config | Config, template |
| question | Questionnaire |
| invariant | I-1 à I-15 |

## @layer

| layer | Utilisation |
|-------|-------|
| core | Noyau |
| phase | Par phase |
| agent | Par agent |
| metric | Métriques |
| config | Config |

## Format (Markdown)

```markdown
<!-- @id mip.p0.t1.brainstorm
     @do exploration_et_questionnaire_structure
     @role orchestration
     @layer phase
     @human Temps 1 — Exploration et Brainstorming -->
```
