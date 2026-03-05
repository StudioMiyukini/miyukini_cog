# Spécification — Audit COG central-improve-secure-update

## TL;DR

Méthodologie audit : scan >400 lignes, classification par zone, axes refactorisation/modularisation/granulation, index MSCM, conformité I-14. Exclusions MGE.

---

## Méthodologie scan

1. **Commandes** : PowerShell `Get-ChildItem -Recurse` sur crates/, apps/, docs/, .mip/
2. **Extensions** : .rs, .md, .ts
3. **Exclusions** : mge/, target/, node_modules/, Allumina/, Sodomight
4. **Seuil** : 400 lignes (règle I-14)
5. **Sortie** : `lignes[TAB]chemin`

---

## Classification axes

| Axe | Description | Exemple |
|-----|-------------|---------|
| Refactorisation | Extraire fonctions, réduire couplage | work_form.rs |
| Modularisation | index + annexes | plan.md + plan-etape-X.md |
| Granulation | sous-modules par responsabilité | ui_builder → ui_builder_*.rs |

---

## Format livrable monolithiques

| Colonne | Type | Exemple |
|---------|------|---------|
| chemin | string | apps/central/src/services/ui_builder.rs |
| lignes | number | 2204 |
| zone | enum | apps, crates, docs, .mip |
| axe | enum | refactorisation, modularisation, granulation |
| priorité | 1-10 | 3 |

---

## Références

- Règle I-14 : `.mip/modules/workflow.md`
- MSCM : `.mip/skills/miyukini-mscm-mip/SKILL.md`
- project-file-map : `.mip/memory/project-file-map.md`
