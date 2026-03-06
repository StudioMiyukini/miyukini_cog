# Index des ressources - 2026-03-06-mge-sodomight

> Sequence T5. Maria a rempli ce fichier avec les ressources de P0 necessaires a la suite.
> Les documents detailes Sodomight/MGE vivent dans `ressources/requirements/`.

## Documentation

- `requirements/00-index.md`
- `requirements/01-gamedesign-pillars.md`
- `requirements/02-core-loop-session-loop.md`
- `requirements/03-progression-classes-skills.md`
- `requirements/04-combat-moment-to-moment.md`
- `requirements/05-itemization-loot-economy.md`
- `requirements/06-world-acts-quests-endgame.md`
- `requirements/07-monsters-bosses-encounters.md`
- `requirements/08-social-modes-ux.md`
- `requirements/09-mge-engine-runtime.md`
- `requirements/10-mge-render-assets-tools.md`
- `requirements/11-central-packaging-ops.md`
- `requirements/12-stats-resistances-breakpoints.md`
- `requirements/13-town-services-cube-crafting.md`
- `requirements/14-mercenaries-party-ladder-hardcore-pvp.md`
- `requirements/15-zones-randomization-waypoints-quests.md`
- `requirements/16-ui-audio-feedback-accessibility.md`
- `requirements/17-content-authoring-asset-generation.md`
- `requirements/18-mvp-camp-act1.md`
- `requirements/19-feature-matrix-d2-mvp.md`
- `requirements/20-act1-content-bible.md`
- `requirements/21-implementation-doc-stack.md`
- `requirements/22-render-modern-robust-scalable.md`
- `requirements/23-sodomight-asset-style-bible.md`
- `requirements/24-mmo-backend-readiness.md`
- `sources-web.md`

## Certifications / Referentiels

- `docs/services/Allumina/Allumina - Analyse Technique Diablo II pour MGE.md`
- `docs/services/Allumina/Allumina - Extraction Systemes D2 OpenDiablo2 pour MGE.md`
- `docs/services/Allumina/Allumina - Blueprint Moteur Sandbox MGE.md`
- `apps/central/src/service_manager/registry.rs`
- `crates/miyumarket/src/manifest.rs`

## Securite (P0 T5 -> P4)

### RPS (obligatoire)

- Surfaces d'attaque et menaces : `phases/p0/temps/temps-05-securite.md`
- Niveau de securite et GPI : `gpi/2026-03-06-mge-sodomight-gpi.md`

### Volet GPI securite (obligatoire)

- Integrite package
- Versioning saves
- Validation des data packs
- Host autoritaire futur

## Librairies / Paquets

- `wgpu`
- `winit`
- `kira`
- `Rapier` si utile, sinon logique custom

## IDs Context7 (verification docs)

- verification docs basee sur les documentations officielles des crates et OpenDiablo2

## Audit securite P4 (PASS -> RAS)

- PASS-0 : `audits/2026-03-06-mge-sodomight-pass-0.md`
- PASS-01 : `audits/2026-03-06-mge-sodomight-pass-01.md`
- RAS : `audits/2026-03-06-mge-sodomight-ras.md`

Regle de rebouclage:
- Breche critique ou score insuffisant -> retour cycle MIP (P0 Temps 1)
