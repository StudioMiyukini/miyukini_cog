# Sodomight - Index des besoins

## Regles de lecture

- Chaque fichier reste sous 400 lignes.
- La structure suit `gamedesign -> boucle -> features -> moteur -> integration`.
- Les besoins D2 sont decrits comme references de comportement.
- Les besoins Sodomight sont des exigences de reproduction ou de modernisation.

## Modules

1. `01-gamedesign-pillars.md`
2. `02-core-loop-session-loop.md`
3. `03-progression-classes-skills.md`
4. `04-combat-moment-to-moment.md`
5. `05-itemization-loot-economy.md`
6. `06-world-acts-quests-endgame.md`
7. `07-monsters-bosses-encounters.md`
8. `08-social-modes-ux.md`
9. `09-mge-engine-runtime.md`
10. `10-mge-render-assets-tools.md`
11. `11-central-packaging-ops.md`
12. `12-stats-resistances-breakpoints.md`
13. `13-town-services-cube-crafting.md`
14. `14-mercenaries-party-ladder-hardcore-pvp.md`
15. `15-zones-randomization-waypoints-quests.md`
16. `16-ui-audio-feedback-accessibility.md`
17. `17-content-authoring-asset-generation.md`
18. `18-mvp-camp-act1.md`
19. `19-feature-matrix-d2-mvp.md`
20. `20-act1-content-bible.md`
21. `21-implementation-doc-stack.md`
22. `22-render-modern-robust-scalable.md`
23. `23-sodomight-asset-style-bible.md`
24. `24-mmo-backend-readiness.md`

## Decision de cadrage

Le moteur est developpe en fonction des besoins du jeu et non l'inverse. Dans cette sequence, le MVP ne signifie pas "sous-ensemble de systems"; il signifie "toutes les fonctionnalites majeures de D2, avec contenu authored borne au camp de depart et a l'Acte 1". Les features qui depassent naturellement ce contenu doivent tout de meme exister au niveau runtime, data, UI et tests. Le rendu doit rester original meme s'il reprend des principes de lisibilite D2, et le backend doit pouvoir sortir du mode solo/coop vers une architecture MMO autoritaire.
