# P0 Temps 06 - Specification technique

## Sorties produites

- `specs/2026-03-06-mge-sodomight-spec.md`
- `specs/2026-03-06-mge-sodomight-spec-engine.md`
- `specs/2026-03-06-mge-sodomight-spec-render.md`
- `specs/2026-03-06-mge-sodomight-spec-central.md`

## Principes actes

- backend 100% Rust
- rendu proprietaire sur `wgpu` + `winit`
- simulation serveur/host autoritaire meme en coop local
- architecture data-driven pour skills, items, monstres, quetes, zones
- separation stricte `engine`, `gameplay`, `tools`, `assets-build`
