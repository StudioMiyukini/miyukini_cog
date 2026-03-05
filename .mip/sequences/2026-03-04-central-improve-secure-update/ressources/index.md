# Ressources — central-improve-secure-update

> Audit complet Miyukini COG (hors MGE). Inventaire, monolithiques, MSCM, conformité, réorganisation.

## Références MIP

| Fichier | Contenu |
|---------|---------|
| `.mip/modules/workflow.md` | I-14 artefacts 400 lignes max, workflow phases |
| `.mip/protocol/conventions.md` | Classification, équipe, artefacts |
| `.mip/memory/project-file-map.md` | Cartographie (à compléter) |
| `.mip/memory/patterns-and-lessons.md` | Anti-patterns, erreurs |
| `.mip/memory/MEMORY.md` | Couverture MSCM, zones |

## MSCM et index

| Fichier | Contenu |
|---------|---------|
| `.mip/skills/miyukini-mscm-mip/SKILL.md` | Protocole MSCM, index |
| `mscm_index/` | Index généré (blocks.json, hierarchy.json, etc.) |
| `tools/mip-generator/` | Générateur MSCM Index |
| `docs/contrats/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md` | Protocole index |

## Zones COG hors MGE

| Zone | Chemin | Contenu |
|------|--------|---------|
| Cores | crates/miyukini-kernel, strongfather, kindmother, worrysentinel... | Strate 1-4 |
| Toolkits | crates/miyuki-ui-*, miyusql, miyauth... | Strate 5-6 |
| Services | crates/miyukini-central, jayfestival, miyucloud... | Strate 7-8 |
| Apps | apps/central, apps/origin, apps/miou-llm-bridge... | Binaires |
| Docs | docs/ | Documentation |
| MIP | .mip/ | Protocole, mémoire, skills |
| Tools | tools/ | mip-generator, etc. |
