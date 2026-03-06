# Spec engine - mge-sodomight

## Workspace cible

- `mge/Cargo.toml`
- `mge/crates/mge-core`
- `mge/crates/mge-ecs`
- `mge/crates/mge-render`
- `mge/crates/mge-audio`
- `mge/crates/mge-input`
- `mge/crates/mge-nav`
- `mge/crates/mge-save`
- `mge/crates/mge-net`
- `mge/crates/mge-proto`
- `mge/crates/mge-server-core`
- `mge/crates/mge-replication`
- `mge/crates/mge-content`
- `mge/games/sodomight`
- `mge/services/mge-login-gateway` futur
- `mge/services/mge-realm` futur
- `mge/services/mge-zone` futur

## API runtime minimale

- `EngineApp`
- `GameModule`
- `World`
- `Schedule`
- `AssetStore`
- `SaveManager`
- `NetMode`
- `AuthoritativeSim`
- `ReplicationBridge`

## Invariants

- tick fixe
- separation simulation / rendu
- chargement des donnees au boot de scene
- type-safe handles pour assets et entites
- serialisation versionnee
- memes regles gameplay en local, host dedie et MMO futur
- aucun calcul critique ne depend du client rendu
