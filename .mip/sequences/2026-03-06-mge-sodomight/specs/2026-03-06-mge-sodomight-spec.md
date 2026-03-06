# Spec - mge-sodomight

## TL;DR

Construire un workspace `mge/` autonome en Rust, avec renderer proprietaire `wgpu`, runtime ARPG data-driven, outils d'assets internes, backend autoritaire evolutif vers MMO, et un premier jeu `Sodomight` integrable dans `Central` comme service standalone.

## Navigation

- `2026-03-06-mge-sodomight-spec-engine.md`
- `2026-03-06-mge-sodomight-spec-render.md`
- `2026-03-06-mge-sodomight-spec-central.md`
- `../ressources/requirements/00-index.md`
- `../ressources/requirements/22-render-modern-robust-scalable.md`
- `../ressources/requirements/24-mmo-backend-readiness.md`

## Decisions clefs

- Pas de moteur tiers complet.
- Pas de dependance directe de `mge/` au workspace racine.
- Pipeline de contenus et de donnees versionne des le debut.
- Parite systemique D2 avant extension de contenu au-dela de l'Acte 1.
- Toute feature D2 hors expression naturelle Acte 1 doit avoir un chemin de validation runtime/tests.
- Rendu D2-like par principes visuels, jamais par reprise directe d'assets.
- Coeur simulation partage entre solo, coop et evolution MMO.

## Out of scope P3 initial

- contenu authored des actes 2 a 5
- exploitation live saisonniere complete
- endgame additionnel post-campagne
- exploitation MMO complete en production
- marketplace d'items
