<!-- @id mem.map.project_file_map
     @do map_project_files_and_areas
     @role index
     @layer memory
     @human Cartographie des fichiers et zones du projet -->

# Cartographie fichiers projet — Miyukini COG (hors MGE)

> Complété 2026-03-04 — Audit central-improve-secure-update.

## Zones principales

| Zone | Chemin | Responsable | Risques |
|------|--------|-------------|---------|
| Cores | crates/miyukini-kernel, strongfather, kindmother, worrysentinel, borderguard... | Denis | Dépendances croisées |
| Toolkits | crates/miyuki-ui-*, miyusql, miyauth, miyuexport... | François/Lise | Couplage UI |
| Services | crates/miyukini-central, jayfestival, miyucloud, jaymanga, jayxpose... | Denis | Monolithiques kindmother_db |
| Apps | apps/central, apps/origin, apps/miou-llm-bridge... | Denis | ui_builder 2204L, pages 3575L |
| Docs | docs/ (hors Allumina/Sodomight MGE) | Maria | Nombreux >400 lignes |
| MIP | .mip/ (memory, skills, modules, config, certifications) | Maria | Conventions |
| Tools | tools/mip-generator | Hugo | — |

**Exclusions** : mge/, sodomight, Allumina, mge-* (workspace MGE).

## Fichiers critiques (>400 lignes)

| Chemin | Lignes | Zone | Axe prioritaire |
|--------|--------|------|-----------------|
| apps/origin/src/web/pages.rs | 3575 | Apps | Modularisation |
| apps/origin/src/web/content.rs | 2440 | Apps | Modularisation |
| apps/central/src/services/ui_builder.rs | 2204 | Apps | Granulation |
| crates/jayxpose/.../e07_vitrine_presentation.rs | 2251 | Services | Refactorisation |
| crates/lord_of_the_castle/loot.rs | 2155 | Services | Granulation |
| docs/reference/...Glossaire.md | 2158 | Docs | Index + annexes |
| apps/central/src/services/mws_view.rs | 1095 | Apps | Granulation |
| apps/central/src/services/jaymanga/work_form.rs | 1024 | Apps | Refactorisation |

> Liste complète : `<sequence>/audits/monolithiques-scan.txt`. Fichiers docs/services/Allumina exclus (MGE).

## Références

- Scan monolithiques : `.mip/sequences/2026-03-04-central-improve-secure-update/audits/monolithiques-scan.txt`
- Plan découpage : `<sequence>/plans_p3/`
- MSCM Index : `mscm_index/`
