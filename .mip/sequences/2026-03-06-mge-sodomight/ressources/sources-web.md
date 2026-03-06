# Sources web consultees

## Diablo II - design et mecaniques

- Diablo Wiki - portail general :
  - https://diablo2.diablowiki.net/Diablo_II
- Diablo Wiki - systemes :
  - https://diablo2.diablowiki.net/Mercenaries
  - https://diablo2.diablowiki.net/Runewords
  - https://diablo2.diablowiki.net/Horadric_Cube
  - https://diablo2.diablowiki.net/Gambling
  - https://diablo2.diablowiki.net/Ladder
  - https://diablo2.diablowiki.net/Hardcore
  - https://diablo2.diablowiki.net/PvP
  - https://diablo2.diablowiki.net/Experience
  - https://diablo2.diablowiki.net/Magic_Find
- diablo2.io - base communautaire :
  - https://diablo2.io/

## Reverse-engineering et impls de reference

- OpenDiablo2 :
  - https://github.com/OpenDiablo2/OpenDiablo2
  - https://pkg.go.dev/github.com/OpenDiablo2/OpenDiablo2
- Spriters Resource - Diablo II / Lord of Destruction :
  - https://www.spriters-resource.com/pc_computer/diablo2diablo2lordofdestruction/

## Briques moteur Rust

- `wgpu` :
  - https://docs.rs/wgpu/latest/wgpu/
- `winit` :
  - https://docs.rs/winit/latest/winit/
- `tokio` :
  - https://tokio.rs/
- `quinn` :
  - https://github.com/quinn-rs/quinn
- `Rapier` :
  - https://rapier.rs/docs/
- `kira` :
  - https://docs.rs/kira/latest/kira/

## Role de ces sources

- Diablo Wiki et diablo2.io servent a stabiliser la taxonomie des features D2.
- OpenDiablo2 sert a verifier les structures de donnees, les entites et les patterns runtime.
- Spriters Resource sert a verifier l'ampleur des familles d'assets D2: classes, NPCs, pieces Rogue, animaux, ennemis/boss, acts, objets, projectiles et effets.
- Les docs Rust servent a verrouiller une architecture moteur low-level sans engine tiers complet.
- `tokio` et `quinn` servent a cadrer une trajectoire backend autoritaire vers coop puis MMO.
